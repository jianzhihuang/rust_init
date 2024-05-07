use actix_web::{get, web::ServiceConfig,web::Path ,HttpResponse, HttpServer, Responder ,web};
use shuttle_actix_web::ShuttleActixWeb;
use std::fs::File;
use std::io::Write;
use rand::Rng;
use rand::distributions::Alphanumeric;
use actix_web::web::Bytes;
use actix_web::http::header;
use rand::rngs::SmallRng;

use rand::SeedableRng;
use std::fs;

use tokio::fs as other_fs;
#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}



#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world)
           .service(hello_world_with_id)
           .default_service(
            // 404 for GET request
            web::route().to(|| async {
                HttpResponse::NotFound().body("Oh no! We couldn't find the route you requested.")
            })
        );
    };

    Ok(config.into())
}
#[get("/{id}/txt")]
async fn hello_world_with_id(id: web::Path<u32>) -> impl Responder {
    let file_size_mb = id.into_inner();
    let file_name = format!("{}.txt", file_size_mb);

    // Check if file already exists
    if fs::metadata(&file_name).is_ok() {
        let content_disposition = format!("attachment; filename={}", file_name);
        return HttpResponse::Ok()
            .header(header::CONTENT_TYPE, "application/octet-stream")
            .header(header::CONTENT_DISPOSITION, content_disposition)
            .body(Bytes::from(fs::read(file_name).expect("Failed to read file")));
    }

    let file_size_bytes: usize = (file_size_mb * 1_000_000).try_into().unwrap();
    let mut file = File::create(&file_name).expect("Failed to create file");

    // Generate data and write to file
    let mut rng = SmallRng::from_entropy();
    for _ in 0..file_size_bytes {
        let c: String = (&mut rng).sample_iter(Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
    
        file.write_all(c.as_bytes()).expect("Failed to write to file");
    }

    let file_size = file.metadata().map(|metadata| metadata.len()).unwrap_or(0);

    let content_disposition = format!("attachment; filename={}", file_name);
    HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .header(header::CONTENT_DISPOSITION, content_disposition)
        .body(Bytes::from(fs::read(file_name).expect("Failed to read file")))
}
async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Oh no! We couldn't find the route you requested.")
}