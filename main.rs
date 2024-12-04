use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Person {
    name: String,
    age: u32,
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello, world!"
}

#[post("/person")]
async fn create_person(person: web::Json<Person>) -> impl Responder {
    format!("Created person: {:?}", person)
}

#[get("/person/{id}")]
async fn get_person(web::Path(id): web::Path<usize>) -> impl Responder {
    format!("Getting person with ID: {}", id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(create_person)
            .service(get_person)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
