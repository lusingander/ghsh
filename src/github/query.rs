use graphql_client::GraphQLQuery;

use crate::github::scalar::DateTime;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/github/schema.docs.graphql",
    query_path = "src/github/query.graphql",
    variables_derives = "Debug",
    response_derives = "Debug"
)]
pub struct UserRepositories;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/github/schema.docs.graphql",
    query_path = "src/github/query.graphql",
    variables_derives = "Debug",
    response_derives = "Debug"
)]
pub struct RepositoryStarHistories;
