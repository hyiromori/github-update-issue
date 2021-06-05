#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
pub struct Owner {
    pub login: String,
    pub owner_type: OwnerType,
}

impl fmt::Display for Owner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{login} ({owner_type})",
            login = self.login,
            owner_type = self.owner_type
        )
    }
}

#[derive(Debug)]
pub enum OwnerType {
    User,
    Organization,
}

impl fmt::Display for OwnerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            OwnerType::User => f.write_str("User"),
            OwnerType::Organization => f.write_str("Organization"),
            _ => f.write_str("Unknown"),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct GitHubIssue {
    pub body: String,
    pub createdAt: String,
    pub number: i32,
    pub title: String,
    pub updatedAt: String,
    pub url: String,
}
