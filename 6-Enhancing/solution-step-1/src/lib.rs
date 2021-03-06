extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub mod error;
mod model;

pub struct RustClient {
    token: String,
}

impl RustClient {
    pub fn new(token: String) -> RustClient {
        RustClient { token }
    }

    pub fn get_pull_requests(&self) -> Result<Vec<model::PullRequest>, error::Error> {
        let uri = format!(
            "https://api.github.com/repos/rust-lang/rust/pulls?access_token={}",
            self.token.trim()
        );

        let body: Vec<model::PullRequest> = reqwest::get(&uri)?.json()?;
        Ok(body)
    }
}

pub fn obtain_token(path: &Path) -> Result<String, error::Error> {
    if path.exists() {
        let mut file = File::open(path)?;
        let mut token = String::new();
        file.read_to_string(&mut token)?;
        Ok(token)
    } else {
        Err(error::Error::TokenNotFound)
    }
}
