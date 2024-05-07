use actix_web::{get, web, Responder};

#[get("/hello/{name}")]
pub async fn basic(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/sum/{num1}/{num2}")]
pub async fn routes(path: web::Path<(u32, u32)>) -> impl Responder {
    let (num1, num2) = path.into_inner();
    format!("{} + {} = {}", num1, num2, num1 + num2)
}
