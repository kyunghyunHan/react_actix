use crate::db::{connection::establish_connection, user::create_post};

use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MyObj {
    data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    first_name: String,
    last_name: String,
    email: String,
}

//joins
pub async fn write_data(info: web::Json<Info>) -> HttpResponse {
    let connection = establish_connection();
    let first_name = &info.first_name.to_string();
    let last_name = &info.last_name.to_string();
    let email = &info.email.to_string();
    let _post = create_post(&connection, first_name, last_name, email);
    HttpResponse::Ok().body(info.first_name.to_string())
}
