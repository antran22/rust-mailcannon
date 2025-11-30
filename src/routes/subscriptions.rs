use actix_web::{Responder, web};

#[derive(serde::Deserialize)]
pub struct CreateSubscriptionData {
    name: String,
    email: String,
}

pub async fn create(form: web::Form<CreateSubscriptionData>) -> impl Responder {
    format!("Hello {}, {}", form.name, form.email)
}
