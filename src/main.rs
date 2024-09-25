mod github;

use github::client::{AccessToken, GhClient};

#[tokio::main]
async fn main() {
    let token = AccessToken::from(std::env::var("GITHUB_ACCESS_TOKEN").unwrap());

    let client = GhClient::new(token);

    client
        .all_user_starred_repositories("lusingander")
        .await
        .unwrap();
}
