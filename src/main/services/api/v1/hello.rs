use std::time::{SystemTime, UNIX_EPOCH};
use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use utoipa::{ToSchema, OpenApi};

use crate::module::greet;

#[derive(Serialize, ToSchema)]
pub struct Greeting {
    /// Unix timestamp when the greeting was created
    timestamp: u32,
    /// Greeting message
    message: String,
}

/// Greeting service
///
/// Returns a personalized greeting message along with the current timestamp.
#[utoipa::path(
    get,
    path = "/api/v1/{name}",
    params(
        ("name" = String, Path, description = "Name of the person to greet. This is a required path parameter.")
    ),
    responses(
        (status = 200, description = "A successful response containing a greeting message and timestamp.", body = Greeting),
        (status = 400, description = "Bad request, typically when no name is provided or invalid input."),
        (status = 500, description = "Internal server error, for unexpected issues.")
    )
)]
#[get("/{name}")]
pub async fn hello_service(name: web::Path<String>) -> impl Responder {
    let greeting_message: String = greet(&name).await;
    let created_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as u32;

    let response = Greeting {
        timestamp: created_at,
        message: greeting_message,
    };

    HttpResponse::Ok().json(response)
}
