use std::{
    error::Error,
    fmt::{Debug, Display},
    time::Duration,
};

use graphql_client::{GraphQLQuery, Response};
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    ClientBuilder,
};

use crate::github::query;

use super::scalar::DateTime;

const GITHUB_GRAPHQL_API_ENDPOINT: &str = "https://api.github.com/graphql";

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
        let user_agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, token.to_header_value());

        let underlying = ClientBuilder::new()
            .user_agent(user_agent)
            .default_headers(headers)
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();

        Self { underlying }
    }
}

impl GhClient {
    pub async fn all_user_starred_repositories(
        &self,
        user: &str,
    ) -> Result<Vec<(String, usize)>, Box<dyn Error>> {
        let mut ret: Vec<(String, usize)> = Vec::new();
        let mut cursor = None;

        loop {
            let resp = self.user_repositories(user, cursor).await?;

            let Some(user) = resp.user else {
                return Err("No user in response".into());
            };

            for edge in user.repositories.edges.unwrap_or_default() {
                if let Some(repository) = edge.and_then(|edge| edge.node) {
                    let name = repository.name;
                    let star = repository.stargazer_count;
                    if star == 0 {
                        break;
                    }
                    ret.push((name, star as usize));
                }
            }

            if !user.repositories.page_info.has_next_page {
                break;
            }
            cursor = user.repositories.page_info.end_cursor;
        }

        for (name, star) in &ret {
            println!("{}: {}", name, star);
        }

        Ok(ret)
    }

    async fn user_repositories(
        &self,
        user: &str,
        cursor: Option<String>,
    ) -> Result<query::user_repositories::ResponseData, Box<dyn Error>> {
        let variables = query::user_repositories::Variables {
            user: user.to_string(),
            first: 50,
            cursor,
        };
        let query = query::UserRepositories::build_query(variables);

        let resp = self
            .underlying
            .post(GITHUB_GRAPHQL_API_ENDPOINT)
            .json(&query)
            .send()
            .await?;

        let resp_body = resp
            .error_for_status()?
            .json::<Response<query::user_repositories::ResponseData>>()
            .await?;

        match resp_body.data {
            Some(data) => Ok(data),
            None => Err(format!("No data in response: {:?}", resp_body).into()),
        }
    }

    pub async fn all_repository_star_histories(
        &self,
        owner: &str,
        name: &str,
    ) -> Result<Vec<DateTime>, Box<dyn Error>> {
        let mut ret: Vec<DateTime> = Vec::new();
        let mut cursor = None;

        loop {
            let resp = self.repository_star_histories(owner, name, cursor).await?;

            let Some(repository) = resp.repository else {
                return Err("No repository in response".into());
            };

            for edge in repository.stargazers.edges.unwrap_or_default() {
                if let Some(starred_at) = edge.map(|edge| edge.starred_at) {
                    ret.push(starred_at);
                }
            }

            if !repository.stargazers.page_info.has_next_page {
                break;
            }
            cursor = repository.stargazers.page_info.end_cursor;
        }

        for date in &ret {
            println!("{:?}", date);
        }

        Ok(ret)
    }

    async fn repository_star_histories(
        &self,
        owner: &str,
        name: &str,
        cursor: Option<String>,
    ) -> Result<query::repository_star_histories::ResponseData, Box<dyn Error>> {
        let variables = query::repository_star_histories::Variables {
            owner: owner.to_string(),
            name: name.to_string(),
            first: 100,
            cursor,
        };
        let query = query::RepositoryStarHistories::build_query(variables);

        let resp = self
            .underlying
            .post(GITHUB_GRAPHQL_API_ENDPOINT)
            .json(&query)
            .send()
            .await?;

        let resp_body = resp
            .error_for_status()?
            .json::<Response<query::repository_star_histories::ResponseData>>()
            .await?;

        match resp_body.data {
            Some(data) => Ok(data),
            None => Err(format!("No data in response: {:?}", resp_body).into()),
        }
    }
}
