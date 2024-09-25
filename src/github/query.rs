use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/github/schema.docs.graphql",
    query_path = "src/github/query.graphql",
    variables_derives = "Debug",
    response_derives = "Debug"
)]
pub struct UserRepositories;
