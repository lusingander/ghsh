use graphql_client::GraphQLQuery;

use crate::github::scalar::DateTime;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/github/graphql/schema.docs.graphql",
    query_path = "src/github/graphql/query.graphql",
    variables_derives = "Debug",
    response_derives = "Debug"
)]
pub struct UserRepositories;

impl user_repositories::Variables {
    pub fn new(user: &str, cursor: Option<String>) -> Self {
        Self {
            user: user.into(),
            first: 50,
            cursor,
        }
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/github/graphql/schema.docs.graphql",
    query_path = "src/github/graphql/query.graphql",
    variables_derives = "Debug",
    response_derives = "Debug"
)]
pub struct RepositoryStarHistories;

impl repository_star_histories::Variables {
    pub fn new(owner: &str, name: &str, cursor: Option<String>) -> Self {
        Self {
            owner: owner.into(),
            name: name.into(),
            first: 100,
            cursor,
        }
    }
}
