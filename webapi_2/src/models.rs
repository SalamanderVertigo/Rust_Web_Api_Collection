use actix_web::{Responder, Error, HttpRequest, HttpResponse};
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
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
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

impl Responder for NewUser {
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
impl NewUser {
    pub async fn create(new_user: NewUser, pool: &PgPool) -> Result<User> {
        let mut tx = pool.begin().await?;
        let user = sqlx::query("INSERT INTO users (first_name, last_name, user_name, password) VALUES ($1, $2, $3, $4) RETURNING id, first_name, last_name, user_name")
            .bind(&new_user.first_name)
            .bind(&new_user.last_name)
            .bind(&new_user.user_name)
            .bind(&new_user.password)
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