use actix_web::{Responder, Error, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize };
use futures::future::{ready, Ready};
use anyhow::Result;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row, types::Uuid};


#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub user_name: String,
    pub password: Option<String>,
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

impl User {
    pub async fn create(user: User, pool: &PgPool) -> Result<User> {
        let mut tx = pool.begin().await?;
        let user = sqlx::query("INSERT INTO users (first_name, last_name, user_name, password) VALUES ($1, $2, $3, $4) RETURNING id, first_name, last_name, user_name, password")
            .bind(&user.id)
            .bind(&user.first_name)
            .bind(&user.last_name)
            .bind(&user.user_name)
            .bind(&user.password)
            .map(|row: PgRow| {
                User { 
                    id: row.get("id"),
                    first_name: row.get("first_name"),
                    last_name: row.get("last_name"),
                    user_name: row.get("user_name"),
                    password: row.get("password"),
                }
            })
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(user)
    }

    pub async fn get(pool: &PgPool, id: Uuid) -> anyhow::Result<User> {
        println!("Get User in Models");
        let mut tx = pool.begin().await?;
        let res = sqlx::query("SELECT * FROM users WHERE id = $1")
            .bind(&id)
            .map(|row: PgRow| {
                User { 
                    id: row.get("id"),
                    first_name: row.get("first_name"),
                    last_name: row.get("last_name"),
                    user_name: row.get("user_name"),
                    password: row.get("password"),
                } 
            })
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(res)
    }

 }