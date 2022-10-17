mod routes;
mod telegram;
mod utils;
mod mailrelay;
mod mailer;

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
    //let _mailrelay_account = env::var("MAILRELAY_ACCOUNT").expect("MAILRELAY_ACCOUNT not set");
    //let _mailrelay_token = env::var("MAILRELAY_TOKEN").expect("MAILRELAY_TOKEN not set");
    let _mail_server = env::var("MAIL_SERVER").expect("MAIL_SERVER not set");
    let _mail_user = env::var("MAIL_USER").expect("MAIL_USER not set");
    let _mail_password = env::var("MAIL_PASSWORD").expect("MAIL_PASSWORD not set");
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
