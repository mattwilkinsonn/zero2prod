use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SubscriptionData {
    email: String,
    name: String,
}

#[tracing::instrument(name = "Adding a new subscriber", skip(form, pool), fields(email = %form.email, name = %form.name))]
pub async fn subscribe(
    form: web::Form<SubscriptionData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    insert_subscriber(&pool, &form)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, form: &SubscriptionData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4);
        ",
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
