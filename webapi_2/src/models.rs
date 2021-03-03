use actix_web::{Responder, Error, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize };
use futures::future::{ready, Ready};
use anyhow::Result;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row};

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub user_name: String,
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
        let user = sqlx::query("INSERT INTO users (id, first_name, last_name, user_name) VALUES ($1, $2, $3, $4) RETURNING id, first_name, last_name, user_name")
            .bind(&user.id)
            .bind(&user.first_name)
            .bind(&user.last_name)
            .bind(&user.user_name)
            .map(|row: PgRow| {
                User {
                    id: row.get(0),
                    first_name: row.get(1),
                    last_name: row.get(2),
                    user_name: row.get(3),
                }
            })
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(user)
    }
}
