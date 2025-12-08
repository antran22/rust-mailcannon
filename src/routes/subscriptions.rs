use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct CreateSubscriptionData {
    name: String,
    email: String,
}

pub async fn create(
    form: web::Form<CreateSubscriptionData>,
    connection: web::Data<PgPool>,
) -> impl Responder {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "adding new subscriber",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );

    let _span_guard = request_span.enter();

    let query_span = tracing::info_span!("saving susbcriber detail");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!("saved new subscriber details");
            HttpResponse::Ok().body("subscription created")
        }
        Err(e) => {
            tracing::error!("error saving new subscriber details {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
