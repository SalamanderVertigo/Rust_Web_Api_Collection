use actix_web::web;
use crate::routes::{account_init};

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/account").configure(account_init));   
}