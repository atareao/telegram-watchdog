mod routes;
mod mailer;
mod telegram;
mod utils;

use dotenv::dotenv;
use std::env;
use actix_web::{App, HttpServer, middleware::Logger};
use env_logger::Env;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let log_level = env::var("LOG_LEVEL").expect("LOG_LEVEL not set");
    let port = env::var("PORT").expect("PORT not set");
    let _token = env::var("TOKEN").expect("TOKEN not set");
    let _server = env::var("SERVER").expect("SERVER not set");
    let _server_username = env::var("SERVER_USERNAME").expect("SERVER_USERNAME not set");
    let _server_password = env::var("SERVER_PASSWORD").expect("SERVER_PASSWORD not set");
    let _from = env::var("FROM").expect("FROM not set");

    env_logger::init_from_env(Env::default().default_filter_or(log_level));

    HttpServer::new(move ||{
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(routes::root)
            .service(routes::status)
            .service(routes::hook)
    })
        .bind(format!("0.0.0.0:{}", &port))
        .unwrap()
        .run()
        .await
}
