use actix_web::{App, HttpServer};
use todo::routes::web::register as web_route;

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(|| App::new().configure(web_route))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;

    Ok(())
}
