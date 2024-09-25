mod github;

use github::client::{AccessToken, GhClient};

#[tokio::main]
async fn main() {
    let token = AccessToken::from(std::env::var("GITHUB_ACCESS_TOKEN").unwrap());

    let client = GhClient::new(token);

    let repos = client
        .all_user_starred_repositories("lusingander")
        .await
        .unwrap();

    for (name, star) in repos {
        println!("{}: {}", name, star)
    }

    let hists = client
        .all_repository_star_histories("lusingander", "stu")
        .await
        .unwrap();

    for hist in hists {
        println!("{:?}", hist);
    }
}
