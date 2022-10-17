use actix_web::{get, post, Error, HttpResponse, http::header::ContentType,
                HttpRequest};
use serde::Serialize;
use serde_json::{Value, json};
use std::{env, str::FromStr};
use log::{info, error};
use crate::{utils::Mail, mailrelay::MailRelay, mailer::Mailer};
use std::thread;

#[derive(Serialize)]
struct Respuesta{
    code: i32,
    status: String,
    content: Value,
}
impl Respuesta {
    fn new(code: i32, content: Value) -> Result<HttpResponse, Error>{
        let respuesta = Respuesta{
            code,
            status: if code < 300 {"OK".to_string()} else {"KO".to_string()},
            content,
        };
        match code{
            0 ..= 299 => Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&respuesta)?)),
            _ => Ok(HttpResponse::BadRequest()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&respuesta)?)),
        }
    }

    fn simple(code: i32, message: &str) -> Result<HttpResponse, Error>{
        Respuesta::new(code, json!({"description": message}))
    }
}

#[get("/")]
pub async fn root(req: HttpRequest) -> Result<HttpResponse, Error>{
    Respuesta::simple(200, "Rust es lo mejor!")
}

#[get("/status")]
pub async fn status(req: HttpRequest) -> Result<HttpResponse, Error>{
    Respuesta::simple(200, "Up and running")
}

#[post("/hook")]
pub async fn hook(req: HttpRequest, content: String) -> Result<HttpResponse, Error>{
    println!("Content: {}", content);
    let post_content = Value::from_str(&content).unwrap();
    match &post_content.get("message"){
        Some(message) => {
            match &message.get("reply_to_message"){
                Some(original) => {
                    println!("Original: {}", original);
                    let original_text = original.get("text").unwrap().as_str().unwrap();
                    println!("Texto original: {}", original_text);
                    let mail = Mail::parse_from(original_text);
                    if mail.is_some(){
                        println!("El mail: {}", mail.unwrap());
                        println!("La respuesta: {}", message.get("text").unwrap());
                    }
                },
                None => {
                    let maybe = message.get("text").unwrap().as_str().unwrap();
                    let mail = Mail::parse_to(maybe);
                    if mail.is_some(){
                        info!("I'm going to send message");
                        thread::spawn(||{
                            let mail_content = mail.unwrap();
                            let server = env::var("MAIL_SERVER").unwrap();
                            let username = env::var("MAIL_USER").unwrap();
                            let password = env::var("MAIL_PASSWORD").unwrap();
                            let mailer = Mailer::new(&server, &username, &password);
                            match mailer.send(&mail_content) {
                                Ok(response) => {
                                    info!("Send message: {}", response.code());
                                },
                                Err(e) => {
                                    error!("No pude enviar el mensaje: {}", e);
                                    error!("Mail: {}", &mail_content);
                                },
                            }

                        });
                        /*
                        let mail_content = mail.unwrap();
                        let account = env::var("MAILRELAY_ACCOUNT").unwrap();
                        let token = env::var("MAILRELAY_TOKEN").unwrap();
                        let mailrelay = MailRelay::new(&account, &token);
                        match mailrelay.send(&mail_content).await{
                            Ok(result) => {
                                info!("Send message: {}", result.status());
                            },
                            Err(e) => {
                                error!("No pude enviar el mensaje: {}", e);
                                error!("Mail: {}", mail_content);
                            }
                        };
                        */
                    }
                },
            }
        },
        None => info!("No es un mensaje"),
    }
    Respuesta::simple(200, "Up and running")
}

