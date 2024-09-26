pub mod client;
mod query;
mod scalar;

use chrono::Local;

use crate::github::scalar::DateTime;

#[derive(Debug)]
pub struct Repository {
    pub name: String,
    #[expect(dead_code)]
    star: usize,
}

impl Repository {
    pub fn new(name: String, star: usize) -> Self {
        Self { name, star }
    }
}

#[derive(Debug)]
pub struct Star {
    pub starred_at: chrono::DateTime<Local>,
}

impl Star {
    pub fn new(starred_at: DateTime) -> Self {
        Self {
            starred_at: starred_at.try_into().unwrap(),
        }
    }
}
