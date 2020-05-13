#[macro_use]
extern crate diesel;
extern crate dotenv;
use serde::{Serialize, Deserialize};
mod db;
use db::*;
extern crate serde_derive;
use actix_web::{web, App, HttpResponse, HttpServer, HttpRequest, Responder};

async fn index() -> impl Responder {
    let posts = get_posts();
    HttpResponse::Ok().json(posts)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .data(web::JsonConfig::default().limit(4096))
        .route("/", web::get().to(index))
        .route("/create", web::post().to(create))
        .route("/publish/{id}", web::put().to(publish))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}

async fn publish(path: web::Path<String>) -> impl Responder {
    let result = publish_post(path.to_string());

    HttpResponse::Ok().json(result)
}

#[derive(Debug, Serialize, Deserialize)]
struct CreatePost {
    title: String,
    body: String,
}

async fn create(post: web::Json<CreatePost>, 
          req: HttpRequest) -> impl Responder {
    println!("request: {:?}", req);
    println!("model: {:?}", post);
    let result = create_post(post.0.title.as_ref(),
                             post.0.body.as_ref());
    HttpResponse::Ok().json(result)
}