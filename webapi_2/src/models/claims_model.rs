use std::fmt;
use serde::{Deserialize, Serialize };
use sqlx::{types::Uuid};

// claims struct is where we store the info of the token we need to validate over the api
#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: Uuid, // subject, or the thing being validated, like a users uuid
    pub role: Role, // roll for the authenticated User
    pub exp: usize, // expiration of the token
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Role {
    Admin,
    User,
}


impl Role {
    pub fn from_str(role: &str) -> Role {
        match role {
          "Admin" => Role::Admin,
            _=> Role::User,
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Admin => write!(f, "Admin"),
        }
    }
}