use actix_web::{dev::ServiceRequest, Error, ResponseError};
use actix_web::error::ErrorUnauthorized;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
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
        Err(_) => panic!("Error creating the token"), // actually return an http response error here
    };
    
    Ok(LoginResponse{jwt: token})
   
}

// TODO Create custom response errors 
pub async fn bearer_token_check(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    println!("Bearer Token Check");
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone()) // had to remove the .get_ref() from data to work
        .unwrap_or_else(Default::default);
    match validate_token(credentials.token()) {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
               /* panic!("PANIC1: {:?}", res);*/
                Err(AuthenticationError::from(config).into()) 
            }
        }
        _ => {
            /*panic!("PANIC2: {:?}", config);*/
           // Err(_) => Err(AuthenticationError::from(config).into()),
        }
    }
}

fn validate_token(token: &str) -> Result<bool, Error> {
    let token_data = match decode::<Claims>(
        &token.to_string(),
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::new(Algorithm::HS512),
    )
     {
         Ok(claims) => Ok(claims),
        Err(err) =>  {
            error!("jwt.decode {} failed: {:?}", token, err);
            Err(format!("invalid token: {}", err).into())
            /*ErrorKind::ExpiredSignature => Err(format!("Expired Signature: {}", err).into()),
            _ => Err(format!("Unknown Error: {}", err).into()),*/
        },
    };
    
   /* println!("TOKEN_DATA_CLAIMS => {:?}", token_data.claims);
    println!("TOKEN_DATA_HEADER => {:?}", token_data.header);*/

    if !token_data.claims.sub.to_string().is_empty()
    { 
        return Ok(true);
    }
    Err(format!("Authorization Denied:").into())
}
