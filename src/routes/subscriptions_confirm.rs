use actix_web::{web, HttpRequest, HttpResponse};

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

#[allow(clippy::async_yields_async)]
#[tracing::instrument(name = "Confirm a pending subscriber", skip(_paramters))]
pub async fn confirm(_paramters: web::Query<Parameters>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
