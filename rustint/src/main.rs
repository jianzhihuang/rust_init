use actix_web::{get, web::ServiceConfig,web::Path};
use shuttle_actix_web::ShuttleActixWeb;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
    };

    Ok(config.into())
}

#[get("/<id>/txt")]
async fn hello_world_with_id(id: Path<u32>) -> &'static str {
    println!("The id is: {}", id);
    "Hello World!"
}
