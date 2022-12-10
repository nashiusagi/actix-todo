use actix_web::web::{get, resource, ServiceConfig};

use crate::controllers::todo_controller::TodoController;

pub fn register(config: &mut ServiceConfig) {
    config.service(resource("/").route(get().to(TodoController::index)));
}
