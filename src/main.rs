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

    for repo in repos {
        println!("{:?}", repo)
    }

    let stars = client
        .all_repository_stars("lusingander", "stu")
        .await
        .unwrap();

    for star in stars {
        println!("{:?}", star);
    }
}
