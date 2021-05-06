use actix_web::{Responder, Error, HttpRequest, HttpResponse};
use argon2::{self, Config};
use serde::{Deserialize, Serialize };
use futures::future::{ready, Ready};
use anyhow::Result;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row, types::Uuid};
use crate::auth;

// Struct to represent database record
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub user_name: String,
}

// struct to receive user input
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct InternalUser {
    pub first_name: String,
    pub last_name: String,
    pub user_name: String,
    pub password: String,
}

// struct to verify user credentials
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct LoginRequest {
    pub user_name: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct LoginResponse {
    pub jwt: String
}

impl Responder for User {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
        )
    }
}

impl Responder for InternalUser {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
        )
    }
}

impl Responder for LoginRequest {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
        )
    }
}

// any methods needed for new user strucs here (new user omits id as it's generated in the db)
impl InternalUser {
    pub async fn create(new_user: InternalUser, pool: &PgPool) -> Result<User> {
        let mut tx = pool.begin().await.unwrap();
        let password = new_user.password.as_bytes();
        let salt = b"randomsalt";
        let config = Config::default();
        let hash = argon2::hash_encoded(password, salt, &config).unwrap();
        let user = sqlx::query("INSERT INTO users (first_name, last_name, user_name, password) VALUES ($1, $2, $3, $4) RETURNING id, first_name, last_name, user_name")
            .bind(&new_user.first_name)
            .bind(&new_user.last_name)
            .bind(&new_user.user_name)
            .bind(&hash)
            .map(|row: PgRow| {
                User { 
                    id: row.get("id"),
                    first_name: row.get("first_name"),
                    last_name: row.get("last_name"),
                    user_name: row.get("user_name"),
                }
            })
            .fetch_one(&mut tx)
            .await
            .unwrap();

        tx.commit()
        .await
        .unwrap();
        Ok(user)
    }   
}

impl LoginRequest {
    pub async fn login(pool: &PgPool, verify_user: LoginRequest) -> Result<LoginResponse, String> {
        let mut tx = pool.begin().await.unwrap();
        let db_verify_user = sqlx::query("SELECT * FROM users WHERE user_name = $1")
            .bind(&verify_user.user_name)
            .map(|row: PgRow| {
                LoginRequest { 
                    user_name: row.get("user_name"),    
                    password: row.get("password")
                } 
            })
            .fetch_one(&mut tx)
            .await.unwrap();

        tx.commit()
        .await
        .unwrap();
        
        let matches = argon2::verify_encoded(&db_verify_user.password, &verify_user.password.as_bytes()).unwrap();

        if matches { 
            let mut tx = pool.begin().await.unwrap();
            let db_user = sqlx::query("SELECT * FROM users WHERE user_name = $1")
                .bind(&verify_user.user_name)
                .map(|row: PgRow| {
                    User { 
                        id: row.get("id"),
                        first_name: row.get("first_name"),
                        last_name: row.get("last_name"),
                        user_name: row.get("user_name"),    
                        
                    } 
                })
                .fetch_one(&mut tx)
                .await.unwrap();

            tx.commit()
            .await
            .unwrap();
        
            // let key =  env::var("SECRET").expect("SECRET not in env file"); //b"my_secret";
            // let u8_key = key.as_bytes();
            return auth::create_jwt(&db_user.id)

        } else { 
            Err(String::from("Login: Failure"))
        }   
    }
}

 // any methoods needed for user struct, user struct omits password as it's not needed to be served up. 
impl User {
    pub async fn get(pool: &PgPool, id: Uuid) -> Result<User> {
        let mut tx = pool.begin().await?;
        let res = sqlx::query("SELECT * FROM users WHERE id = $1")
            .bind(&id)
            .map(|row: PgRow| {
                User { 
                    id: row.get("id"),
                    first_name: row.get("first_name"),
                    last_name: row.get("last_name"),
                    user_name: row.get("user_name"),    
                } 
            })
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(res)
    }
}