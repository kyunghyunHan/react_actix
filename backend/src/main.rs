extern crate actix_web;
#[macro_use]
extern crate diesel;
use actix_cors::Cors;
extern crate emoji_logger;
use actix_web::{http, middleware, web, App, HttpServer};

mod db;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🧑‍🔬 Sample Service Starting");
    std::env::set_var("RUST_LOG", "actix_web=info");
    emoji_logger::init();
    let data = web::Data::new(db::connection::establish_connection);
    let result = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["POST", "GET"])
            .allow_any_header()
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(web::JsonConfig::default().limit(4096))
            .app_data(data.clone())
            .wrap(middleware::Logger::default())
            .route("/crate", web::post().to(routes::user::write_data))
            .route("/read", web::get().to(routes::user::get_data))
        // .route("/update", web::get().to(routes::user::get_data))
        // .route("/delete", web::get().to(routes::user::get_data))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await;
    println!("🧑‍🔬 Sample Service Stopping");
    result
}
