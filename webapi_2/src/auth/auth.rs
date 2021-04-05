use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web::{Error, dev::ServiceRequest};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{encode, decode, Header, Algorithm, DecodingKey, Validation, EncodingKey};
use crate::models::{Claims, LoginResponse};
use sqlx::{types::Uuid};
use chrono::prelude::*;

// const key: String = env::var("SIGNING_KEY").expect("SECRET not in env file");
// const u8_key: &[u8] = key.as_bytes();
const JWT_SECRET: &[u8] = b"secret";

// create the jwt in here, UUID may need to be changed from &str to Uuid type
pub fn create_jwt(uuid: &Uuid) -> Result<LoginResponse, String> {
    let mut header = Header::default();
    header.kid = Some("singing_key".to_owned());
    header.alg = Algorithm::HS512;

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: uuid.to_owned(),
        exp: expiration as usize,
    };

    // set up headers!!
    let token = match encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET)) {
        Ok(t) =>  t,
        Err(_) => panic!("Error creating the token"), // in practice you would return the error
    };
    
    Ok(LoginResponse{jwt: token})
   
}

pub async fn bearer_token_check(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> { 
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone()) // had to remove the .get_ref() from data to work
        .unwrap_or_else(Default::default);
    match validate_token(credentials.token()) {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

fn validate_token(token: &str) -> Result<bool, std::io::Error>
{
    let token_data = match decode::<Claims>(
        &token.to_string(),
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::new(Algorithm::HS512),
    )
     {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!("Invalid Token! {:?}", err),
            _ => panic!("Unknown Error: {:?}", err),
        },
    };
    println!("TOKEN_DATA_CLAIMS => {:?}", token_data.claims);
    println!("TOKEN_DATA_HEADER => {:?}", token_data.header);

    if !token_data.claims.sub.to_string().is_empty()
    {
        return Ok(true);
    }
    return Err(std::io::Error::new(std::io::ErrorKind::Other, "Authentication failed!"));
}
