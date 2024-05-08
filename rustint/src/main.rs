
use std::fs::File;
use std::io::Write;
use rand::Rng;
use rand::distributions::Alphanumeric;

use rand::rngs::SmallRng;

use rand::SeedableRng;
use std::fs;
use tokio::fs as other_fs;
use axum::{extract::Path, response::IntoResponse, Router};

use axum::routing::get;
use tower_http::services::{ServeDir};
use tower_http::services::ServeFile;
use axum::http::StatusCode; // 加入這行
use axum::response::Response;
use std::io::Read;

use tower_http::classify;
async fn hello_world() -> &'static str {
    "Hello, world!"
}


#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
   let router = Router::new()
    .route_service("/", ServeDir::new("static")
        .not_found_service(ServeFile::new("static/index.html") 
     ))
     .route("/hello/:id/:type", get(hello_world_with_id))
     .route("/rand/:id", get(get_rand));

    Ok(router.into())
}
//入參 id
async fn hello_world_with_id(Path((id, type_)): Path<(String, String)>) -> impl IntoResponse {
    match type_.as_str() {
        "heart" => {
            let count = id.parse::<usize>().unwrap_or(1); // 如果轉換失敗，則生成一個愛心符號
            "❤️".repeat(count)
        },
        "smile" =>{
            let count = id.parse::<usize>().unwrap_or(1); // 如果轉換失敗，則生成一個笑臉符號
            "😄".repeat(count)
        }
        "text" => format!("Hello, world! Your ID is {}  {}", id, type_),
        _ => format!("Hello, world! Your ID is {}  {}", id, type_),
    }
}


async fn get_rand(id: Path<u32>) -> impl IntoResponse {
    let file_size_mb = *id;
    let file_name = format!("{}.txt", file_size_mb);

    // Check if file already exists
    if fs::metadata(&file_name).is_ok() {
        let content_disposition = format!("attachment; filename={}", file_name);
        let mut file = fs::File::open(&file_name).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let body = axum::body::Body::from(buffer); // 將 std::io::Bytes<_> 轉換為 hyper::Body

        return Response::builder()
            .header("Content-Disposition", content_disposition)
            .body(body) // 使用 hyper::Body
            .unwrap();
    }

    let mut file = File::create(&file_name).expect("Failed to create file");

    for _ in 0..file_size_mb {
        let data: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(1_000_000) // 生成約 1MB 的數據
        .map(char::from)
        .collect::<Vec<_>>()
        .chunks(30)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");

        file.write_all(data.as_bytes()).expect("Failed to write to file");
    }

    let file_size = file.metadata().map(|metadata| metadata.len()).unwrap_or(0);

    let content_disposition = format!("attachment; filename={}", file_name);
    axum::response::Response::builder()
    .status(StatusCode::OK)
    .header("Content-Type", "application/octet-stream")
    .header("Content-Disposition", content_disposition)
    .body(axum::body::Body::from(fs::read(&file_name).expect("Failed to read file"))) // 使用 axum::body::Body
    .unwrap()
}
