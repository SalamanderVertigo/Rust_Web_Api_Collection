use actix_web::web;
use crate::routes::{account_init};

// handle token checking in here, migrate login function to auth init mod

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/account").configure(account_init));   
}