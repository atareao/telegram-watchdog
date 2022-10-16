use actix_web::{get, post, put, delete, web, Error, HttpResponse, http::StatusCode,
                http::header::ContentType, HttpRequest,
                error::{ErrorBadRequest, ErrorNotFound}};
use serde::Serialize;
use serde_json::{Value, json};
use std::{env, str::FromStr, thread};
use crate::{utils::Mail, mailer::Mailer};
use log::{info, warn, error};

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
                        let server = env::var("SERVER").unwrap();
                        let server_username = env::var("SERVER_USERNAME").unwrap();
                        let server_password = env::var("SERVER_PASSWORD").unwrap();
                        let mailer = Mailer::new(&server, &server_username, &server_password);
                        thread::spawn(move || {
                            info!("I'm going to send message");
                            let mail_content = mail.unwrap();
                            match mailer.send(&mail_content){
                                Ok(result) => {
                                    info!("Send message: {}", result.code());
                                },
                                Err(e) => {
                                    error!("Mail: {}", mail_content);
                                    error!("No pude enviar el mensaje: {}", e)
                                }
                            };
                        });
                    }
                },
            }
        },
        None => info!("No es un mensaje"),
    }
    Respuesta::simple(200, "Up and running")
}

