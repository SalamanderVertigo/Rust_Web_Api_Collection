use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::routes::{account_init, signup_signin_init};
use crate::auth;

// handle token checking in here, migrate routes which require auth function to auth init mod
pub fn config_app(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(auth::bearer_token_check);
    cfg.service(
        web::scope("/account")
        .wrap(auth)
        .configure(account_init)
    );
    cfg.service(web::scope("/api").configure(signup_signin_init))  ;
}