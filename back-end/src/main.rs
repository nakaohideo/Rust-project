mod api;
mod models;
mod repository;

use actix_web::{get,post, web::Data, App, HttpResponse, HttpServer, Responder,http::header};
use actix_cors::Cors;
use api::user_api::{sign_up,sign_in};
use repository::mongodb_repo::MongoRepo;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db=MongoRepo::init().await;
    let db_data=Data::new(db);
    println!("Connected MongoDB");
    HttpServer::new(move || {
      App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(db_data.clone())
            .service(hello)
            .service(echo)
            .service(sign_up)
            .service(sign_in)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}