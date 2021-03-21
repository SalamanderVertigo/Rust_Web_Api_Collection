use actix_web::{Responder, Error, HttpRequest, HttpResponse};
use argon2::{self, Config};
use serde::{Deserialize, Serialize };
use futures::future::{ready, Ready};
use anyhow::Result;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row, types::Uuid};

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
pub struct VerifyUser {
    pub user_name: String,
    pub password: String,
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

impl Responder for VerifyUser {
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
        let mut tx = pool.begin().await?;
        let password = new_user.password.as_bytes();
        let salt = b"randomsalt";
        let config = Config::default();
        let hash = argon2::hash_encoded(password, salt, &config).unwrap();
        // let matches = argon2::verify_encoded(&hash, password).unwrap();
        // assert!(matches);
        let user = sqlx::query("INSERT INTO users (first_name, last_name, user_name, password) VALUES ($1, $2, $3, $4) RETURNING id, first_name, last_name, user_name")
            .bind(&new_user.first_name)
            .bind(&new_user.last_name)
            .bind(&new_user.user_name)
            .bind(&hash)
            .map(|row: PgRow| {
                User { 
                    id: row.get(0),
                    first_name: row.get(1),
                    last_name: row.get(2),
                    user_name: row.get(3),
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

 impl VerifyUser {
    pub async fn login(pool: &PgPool, verify_user: VerifyUser) -> Result<bool> {
        let mut tx = pool.begin().await?;
        let user = sqlx::query("SELECT * FROM users WHERE user_name = $1")
            .bind(&verify_user.user_name)
            .map(|row: PgRow| {
                VerifyUser { 
                    user_name: row.get("user_name"),    
                    password: row.get("password")
                } 
            })
            .fetch_one(&mut tx)
            .await?;

        tx.commit()
        .await
        .unwrap();
        
        // implement the jwt token and seesioning in handler
        let matches = argon2::verify_encoded(&user.password, &verify_user.password.as_bytes()).unwrap();
        Ok(matches)
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