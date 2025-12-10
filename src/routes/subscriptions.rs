use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{self, NewSubscriber};

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
) -> actix_web::Result<impl Responder> {
    let subscriber_name =
        domain::SubscriberName::parse(form.0.name).map_err(actix_web::error::ErrorBadRequest)?;

    let new_subscriber = NewSubscriber {
        email: form.0.email,
        name: subscriber_name,
    };

    let result = insert_subscription(&new_subscriber, &connection).await;

    result
        .map(|_| HttpResponse::Ok().body("subscription created"))
        .map_err(|_| actix_web::error::ErrorInternalServerError("query error"))
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, connection)
)]
async fn insert_subscription(
    new_subscriber: &NewSubscriber,
    connection: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subscriber.email,
        new_subscriber.name.as_ref(),
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
