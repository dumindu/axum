use utoipa::{OpenApi, openapi::OpenApi as OpenApiDoc};

#[derive(OpenApi)]
#[openapi(
    info(title = "Book Service API", version = "1.0.0"),
    servers(
        (url = "http://localhost:3001", description = "Development")
    ),
    components(schemas(
        crate::models::Book,
        crate::errors::ErrorResponse,
        crate::app::shared::ValidationErrorResponse
    )),
)]
pub struct ApiDoc;

pub fn generate_doc() -> OpenApiDoc {
    let mut openapi = ApiDoc::openapi();
    openapi.merge(crate::app::book::BookApi::openapi());
    openapi
}
