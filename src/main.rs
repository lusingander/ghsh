mod chart;
mod github;
mod macros;
mod tui;

use std::{convert::identity, error::Error, fs::File};

use clap::{command, Parser};
use log::LevelFilter;
use simplelog::{format_description, ConfigBuilder, WriteLogger};

use crate::{
    github::{
        client::{AccessToken, GhClient},
        Star,
    },
    tui::{App, Stars},
};

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(short, long, value_name = "NAME")]
    user: Option<String>,

    #[arg(short, long, value_name = "NAME")]
    repository: Option<Vec<String>>,

    #[arg(short, long)]
    token: Option<String>,

    #[arg(long)]
    debug: bool,
}

const GITHUB_ACCESS_TOKEN_ENV_VAR: &str = "GHSH_ACCESS_TOKEN";

fn load_token(token: Option<String>) -> AccessToken {
    if let Some(token) = token {
        return AccessToken::from(token);
    }
    if let Ok(token) = std::env::var(GITHUB_ACCESS_TOKEN_ENV_VAR) {
        return AccessToken::from(token);
    }
    panic!(
        "Failed to load access token from environment variable {}",
        GITHUB_ACCESS_TOKEN_ENV_VAR
    );
}

enum Mode {
    User(String),
    Repositories(Vec<(String, String)>),
}

impl Mode {
    fn new(user: Option<String>, repositories: Option<Vec<String>>) -> Self {
        match (user, repositories) {
            (Some(user), None) => Self::User(user),
            (None, Some(repos)) => Self::Repositories(
                repos
                    .into_iter()
                    .map(|repo| {
                        let parts: Vec<&str> = repo.split('/').collect();
                        if parts.len() != 2 {
                            panic!("Invalid repository format: expected 'user/repo'");
                        }
                        (parts[0].into(), parts[1].into())
                    })
                    .collect(),
            ),
            (Some(user), Some(repos)) => Self::Repositories(
                repos
                    .into_iter()
                    .map(|repo| {
                        if repo.contains('/') {
                            panic!("Invalid repository format: expected not to contain '/' if user is specified");
                        }
                        (user.clone(), repo)
                    })
                    .collect(),
            ),
            _ => panic!("Invalid arguments: either user or repository must be specified"),
        }
    }
}

async fn fetch_all_stars(client: GhClient, mode: &Mode) -> Result<Stars, Box<dyn Error>> {
    let stars = match mode {
        Mode::User(user) => {
            let repos = client.all_user_starred_repositories(user).await?;
            let mut stars: Vec<Star> = Vec::new();
            for repo in repos {
                let ss = client.all_repository_stars(user, &repo.name).await?;
                stars.extend(ss);
            }
            stars.sort_by(|a, b| a.starred_at.cmp(&b.starred_at));
            Stars::User(stars)
        }
        Mode::Repositories(repos) => {
            let mut stars: Vec<(String, Vec<Star>)> = Vec::new();
            for (user, repo) in repos {
                let name = format!("{}/{}", user, repo);
                let ss = client.all_repository_stars(user, repo).await?;
                stars.push((name, ss));
            }
            Stars::Repositories(stars)
        }
    };
    Ok(stars)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.debug {
        let config = ConfigBuilder::new()
            .set_time_format_custom(format_description!(
                "[hour]:[minute]:[second].[subsecond digits:3]"
            ))
            .set_time_offset_to_local()
            .unwrap_or_else(identity)
            .build();
        let log_file = File::create("ghsh.log")?;
        WriteLogger::init(LevelFilter::Debug, config, log_file)?;
    }

    let mode = Mode::new(args.user, args.repository);
    let token = load_token(args.token);
    let client = GhClient::new(token);

    let stars = fetch_all_stars(client, &mode).await?;

    let terminal = ratatui::init();

    let ret = App::new(stars).run(terminal);

    ratatui::restore();
    ret
}
