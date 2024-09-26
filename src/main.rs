mod github;

use clap::{command, Parser};

use crate::github::{
    client::{AccessToken, GhClient},
    Star,
};

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(short, long, value_name = "NAME")]
    user: Option<String>,

    #[arg(short, long, value_name = "NAME")]
    repository: Option<String>,

    #[arg(short, long)]
    token: Option<String>,
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
    Repository(String, String),
}

impl Mode {
    fn new(user: Option<String>, repository: Option<String>) -> Self {
        match (user, repository) {
            (Some(user), None) => Self::User(user),
            (None, Some(repo)) => {
                let parts: Vec<&str> = repo.split('/').collect();
                if parts.len() != 2 {
                    panic!("Invalid repository format: expected 'user/repo'");
                }
                Self::Repository(parts[0].into(), parts[1].into())
            }
            (Some(user), Some(repo)) => Self::Repository(user, repo),
            _ => panic!("Invalid arguments: either user or repository must be specified"),
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mode = Mode::new(args.user, args.repository);
    let token = load_token(args.token);
    let client = GhClient::new(token);

    match mode {
        Mode::User(user) => {
            let repos = client.all_user_starred_repositories(&user).await.unwrap();
            let mut stars: Vec<Star> = Vec::new();
            for repo in repos {
                let ss = client
                    .all_repository_stars(&user, &repo.name)
                    .await
                    .unwrap();
                stars.extend(ss);
            }
            stars.sort_by(|a, b| a.starred_at.cmp(&b.starred_at));
            for star in stars {
                println!("{:?}", star);
            }
        }
        Mode::Repository(user, repo) => {
            let stars = client.all_repository_stars(&user, &repo).await.unwrap();
            for star in stars {
                println!("{:?}", star);
            }
        }
    }
}
