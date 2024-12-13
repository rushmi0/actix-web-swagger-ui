mod index;
mod hello;

use index::index_service;
use hello::hello_service;

use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(hello::hello_service),
    components(schemas(hello::Greeting)),
    info(
        title = "My API อ่านหาพ่องงง มึงเหรอ",
        version = "0.1.0",
        description = "his is a sample API that demonstrates functionality for greeting users."
    )
)]
struct ApiDoc;

pub fn service_hub(cfg: &mut web::ServiceConfig) {
    let api_doc = ApiDoc::openapi();
    cfg.service(
        web::scope("/api/v1")
            .service(index_service)
            .service(hello_service)
    );
    cfg.service(
        SwaggerUi::new("/doc/v1/{_:.*}")
            .url("/doc-api-v1/openapi.json", api_doc.clone())
    );
}
