use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct CreateSubscriptionData {
    name: String,
    email: String,
}

#[tracing::instrument(
    name = "adding a new subscriber",
    skip(form, connection),
    fields(
        subscriber_email=%form.email,
        subscriber_name=%form.name
    )
)]
pub async fn create(
    form: web::Form<CreateSubscriptionData>,
    connection: web::Data<PgPool>,
) -> impl Responder {
    let result = insert_subscription(&form, &connection).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("subscription created"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, connection)
)]
async fn insert_subscription(
    form: &CreateSubscriptionData,
    connection: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection)
    .await
    .map_err(|e| {
        tracing::error!("query failed {:?}", e);
        e
    })?;

    Ok(())
}
