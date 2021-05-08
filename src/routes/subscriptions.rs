use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::{PgPool, query};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}


// Test with
//  curl -v --data-urlencode "name=le guin" --data-urlencode "email=ursela_le_guin@gmail.com" -XPOST -H "Content-type: application/x-www-form-urlencoded" -v 'http://localhost:8000/subscriptions'
pub async fn subscribe(
    form: web::Form<FormData>,
    // Retrieving a connection from the application state!
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    query!(
        r#"
INSERT INTO subscriptions (id, email, name, subscribed_at)
VALUES ($1, $2, $3, $4)
"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().finish())
}
