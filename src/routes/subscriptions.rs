use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use sqlx::PgPool;
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
    .await
    {
        Ok(_) => HttpResponse::Ok().body("subscription created"),
        Err(e) => {
            println!("failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
