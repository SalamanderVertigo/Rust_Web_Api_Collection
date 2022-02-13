use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web::{dev::ServiceRequest, Error};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{encode, decode, Header, Algorithm, DecodingKey, Validation, EncodingKey};
use crate::models::account_model::{LoginResponse};
use crate::models::claims_model::{Claims, Role};
use sqlx::{types::Uuid};
use chrono::prelude::*;
use log::error;

// const key: String = env::var("SIGNING_KEY").expect("SECRET not in env file");
// const u8_key: &[u8] = key.as_bytes();
// move to env file and convert to bytes
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
        role: Role::Admin,
        exp: expiration as usize,
    };

    // set up headers!!
    let token = match encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET)) {
        Ok(t) =>  t,
        Err(_) => panic!("Error creating the token"), 
    };
    
    Ok(LoginResponse{jwt: token})
   
}

pub async fn bearer_token_check(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    println!("Bearer Token Check");
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone()) // had to remove the .get_ref() from data to work
        .unwrap_or_else(Default::default);
    match validate_token(credentials.token()) {
        Ok(res) => {
            println!("Response: {:?}", res);
            if res == true {
                Ok(req)
            } else {
                println!("Error captured?");
                Err(AuthenticationError::from(config).into()) 
            }
        }
        Err(e) => {
            println!("Error_ (ln61): {:?}", e);
            println!("Error_ (ln62): {:?}", config);
            Err(AuthenticationError::from(config).into())
        },
    }
}

fn validate_token(token: &str) -> Result<bool, ErrorKind>
{
    let token_data = match decode::<Claims>(
        &token.to_string(),
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::new(Algorithm::HS512),
    )
     {
        Ok(c) => c,
        Err(err) => return match *err.kind() {
            ErrorKind::InvalidToken => {
                println!("Error 1 -> {:?}", err);
                Err(ErrorKind::InvalidToken)
            },
            ErrorKind::ExpiredSignature => {
                println!("Error 2 ->: {:?}", err);
                Err(ErrorKind::ExpiredSignature)
            }
            _ => {
                println!("Error -> 3: {:?}", err);
                Err(ErrorKind::InvalidIssuer)
            }
        },
    };
    println!("TOKEN_DATA_CLAIMS => {:?}", token_data.claims);
    println!("TOKEN_DATA_HEADER => {:?}", token_data.header);

    if !token_data.claims.sub.to_string().is_empty()
    {
        return Ok(true);
    }
    return Err(ErrorKind::ImmatureSignature);
}
