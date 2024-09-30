use std::{
    error::Error,
    fmt::{Debug, Display},
    time::Duration,
};

use graphql_client::{GraphQLQuery, Response};
use log::debug;
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    ClientBuilder,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::github::{query, Repository, Star};

const GITHUB_GRAPHQL_API_ENDPOINT: &str = "https://api.github.com/graphql";

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct AccessToken(String);

impl AccessToken {
    fn to_header_value(&self) -> HeaderValue {
        let mut value = HeaderValue::from_str(format!("Bearer {}", self.0).as_str()).unwrap();
        value.set_sensitive(true);
        value
    }
}

impl From<String> for AccessToken {
    fn from(token: String) -> Self {
        Self(token)
    }
}

impl Debug for AccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("AccessToken(***)")
    }
}

impl Display for AccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("AccessToken(***)")
    }
}

pub struct GhClient {
    underlying: reqwest::Client,
}

impl GhClient {
    pub fn new(token: AccessToken) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, token.to_header_value());

        let underlying = ClientBuilder::new()
            .user_agent(USER_AGENT)
            .default_headers(headers)
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();

        debug!("GhClient created: {:?}", underlying);

        Self { underlying }
    }
}

impl GhClient {
    pub async fn all_user_starred_repositories(&self, user: &str) -> Result<Vec<Repository>> {
        let mut ret: Vec<Repository> = Vec::new();
        let mut cursor = None;

        loop {
            let resp = self.user_repositories(user, cursor).await?;

            let Some(user) = resp.user else {
                return Err("No user in response".into());
            };

            for repository in user
                .repositories
                .nodes
                .unwrap_or_default()
                .into_iter()
                .flatten()
            {
                let name = repository.name;
                let star = repository.stargazer_count;
                if star == 0 {
                    break;
                }
                ret.push(Repository::new(name, star as usize));
            }

            if !user.repositories.page_info.has_next_page {
                break;
            }
            cursor = user.repositories.page_info.end_cursor;
        }

        Ok(ret)
    }

    async fn user_repositories(
        &self,
        user: &str,
        cursor: Option<String>,
    ) -> Result<query::user_repositories::ResponseData> {
        let variables = query::user_repositories::Variables::new(user, cursor);
        let query = query::UserRepositories::build_query(variables);
        self.request_query(query).await
    }

    pub async fn all_repository_stars(&self, owner: &str, name: &str) -> Result<Vec<Star>> {
        let mut ret: Vec<Star> = Vec::new();
        let mut cursor = None;

        loop {
            let resp = self.repository_star(owner, name, cursor).await?;

            let Some(repository) = resp.repository else {
                return Err("No repository in response".into());
            };

            for edge in repository.stargazers.edges.unwrap_or_default() {
                if let Some(starred_at) = edge.map(|edge| edge.starred_at) {
                    ret.push(Star::new(starred_at));
                }
            }

            if !repository.stargazers.page_info.has_next_page {
                break;
            }
            cursor = repository.stargazers.page_info.end_cursor;
        }

        Ok(ret)
    }

    async fn repository_star(
        &self,
        owner: &str,
        name: &str,
        cursor: Option<String>,
    ) -> Result<query::repository_stars::ResponseData> {
        let variables = query::repository_stars::Variables::new(owner, name, cursor);
        let query = query::RepositoryStars::build_query(variables);
        self.request_query(query).await
    }

    async fn request_query<Query, ResponseData>(&self, query: Query) -> Result<ResponseData>
    where
        Query: Serialize + Debug,
        ResponseData: DeserializeOwned + Debug,
    {
        let req = self
            .underlying
            .post(GITHUB_GRAPHQL_API_ENDPOINT)
            .json(&query)
            .build()?;

        debug!("Request: {:?}", req);
        debug!("Query: {:?}", query);

        let resp = self.underlying.execute(req).await?;

        debug!("Response: {:?}", resp);

        let resp_body = resp
            .error_for_status()?
            .json::<Response<ResponseData>>()
            .await?;

        match resp_body.data {
            Some(data) => Ok(data),
            None => Err(format!("No data in response: {:?}", resp_body).into()),
        }
    }
}
