use crate::models::Todo;
use actix_web::{HttpResponse, ResponseError};
use askama::Template;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use thiserror::Error;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<crate::models::Todo>,
}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}

fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub struct TodoController {}

impl ResponseError for MyError {}

impl TodoController {
    pub async fn index() -> Result<HttpResponse, MyError> {
        use crate::schema::todos::dsl::*;

        let connection = &mut establish_connection();
        let entries = todos
            .limit(5)
            .load::<Todo>(connection)
            .expect("Error loading todos");
        let html = IndexTemplate { entries };
        let response_body = html.render()?;
        Ok(HttpResponse::Ok()
            .content_type("text/html")
            .body(response_body))
    }
}
