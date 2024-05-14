use std::fs::File;
use std::io::Write;

use rand::rngs::SmallRng;

use axum::{extract::Path, response::IntoResponse, Router};
use rand::SeedableRng;
use std::fs;
use tokio::fs as other_fs;

use axum::http::StatusCode; // 加入這行
use axum::response::Response;
use axum::routing::get;
use memmap2::MmapMut;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::fs::OpenOptions;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::spawn;
use std::time::Instant;
use tokio::task;
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route_service(
            "/",
            ServeDir::new("static").not_found_service(ServeFile::new("static/index.html")),
        )
        .route("/hello/:id/:type", get(hello_world_with_id))
        .route("/rand/:id", get(get_rand_txt));

    Ok(router.into())
}
//入參 id
async fn hello_world_with_id(Path((id, type_)): Path<(String, String)>) -> impl IntoResponse {
    match type_.as_str() {
        "heart" => {
            let count = id.parse::<usize>().unwrap_or(1); // 如果轉換失敗，則生成一個愛心符號
            let hearts = vec!["❤️", "♡", "💖", "💟", "🎁"];
            let idx = rand::thread_rng().gen_range(0..hearts.len());
            hearts[idx].repeat(count)
        }
        "smile" => {
            let count = id.parse::<usize>().unwrap_or(1); // 如果轉換失敗，則生成一個愛心符號
            let hearts = vec!["😀", "🤩", "😊", "🙂", "☺️", "😋"];
            let idx = rand::thread_rng().gen_range(0..hearts.len());
            hearts[idx].repeat(count)
        }
        "cry" => {
            let count = id.parse::<usize>().unwrap_or(1); // 如果轉換失敗，則生成一個愛心符號
            let hearts = vec!["😢", "😭", "😿"];
            let idx = rand::thread_rng().gen_range(0..hearts.len());
            hearts[idx].repeat(count)
        }
        "cat" => {
            let count = id.parse::<usize>().unwrap_or(1); // 如果轉換失敗，則生成一個愛心符號
            let hearts = vec!["🐈", "😾", "🐱", "😻", "🐱‍🚀"];
            let idx = rand::thread_rng().gen_range(0..hearts.len());
            hearts[idx].repeat(count)
        }
        "dog" => {
            let count = id.parse::<usize>().unwrap_or(1); // 如果轉換失敗，則生成一個愛心符號
            let hearts = vec!["🐶", "🐕", "🦮", "🐩", "🐕‍🦺"];
            let idx = rand::thread_rng().gen_range(0..hearts.len());
            hearts[idx].repeat(count)
        }

        "pig" => {
            let count = id.parse::<usize>().unwrap_or(1); // 如果轉換失敗，則生成一個愛心符號
            let hearts = vec!["🐷", "🐽", "🐖", "🐗"];
            let idx = rand::thread_rng().gen_range(0..hearts.len());
            hearts[idx].repeat(count)
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

        file.write_all(data.as_bytes())
            .expect("Failed to write to file");
    }

    let file_size = file.metadata().map(|metadata| metadata.len()).unwrap_or(0);

    let content_disposition = format!("attachment; filename={}", file_name);
    axum::response::Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/octet-stream")
        .header("Content-Disposition", content_disposition)
        .body(axum::body::Body::from(
            fs::read(&file_name).expect("Failed to read file"),
        )) // 使用 axum::body::Body
        .unwrap()
}
pub async fn get_rand_txt(id: Path<u32>) -> impl IntoResponse {
    let file_size_mb = *id;
    let file_name = format!("{}.txt", file_size_mb);

    // Check if file already exists
    if fs::metadata(&file_name).is_ok() {
        let content_disposition = format!("attachment; filename={}", file_name);
        let body = axum::body::Body::from(fs::read(&file_name).expect("Failed to read file"));

        return Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/octet-stream")
            .header("Content-Disposition", content_disposition)
            .body(body)
            .unwrap();
    }

    let start_time = Instant::now(); // 獲取開始時間
    let file_size = file_size_mb * 1024 * 1024;

    // 创建并设置文件大小
    let file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&file_name)
    {
        Ok(file) => file,
        Err(_) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(axum::body::Body::from("Failed to open file"))
                .unwrap()
        }
    };

    if let Err(_) = file.set_len(file_size.into()) {
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(axum::body::Body::from("Failed to set file length"))
            .unwrap();
    }

    // 内存映射文件
    let mmap = unsafe { MmapMut::map_mut(&file).expect("無法將檔案映射到記憶體") };
    let mmap = Arc::new(Mutex::new(mmap));

    // 使用多线程批量生成数据
    let num_threads = 4; // 增加線程數量
    let chunk_size = (file_size as usize) / num_threads;
    let handles: Vec<_> = (0..num_threads)
        .map(|i| {
            let mmap = Arc::clone(&mmap);
            let start = i * chunk_size;
            let end = if i == num_threads - 1 {
                file_size as usize
            } else {
                (i + 1) * chunk_size
            };

            task::spawn(async move {
                let mut rng = thread_rng();
                let mut offset = start;
                let mut buffer: Vec<u8> = Vec::with_capacity(chunk_size);

                // 按照 30 行的规律填充数据
                while offset < end {
                    let line: String = (0..30).map(|_| rng.sample(Alphanumeric) as char).collect();
                    buffer.extend_from_slice(line.as_bytes());
                    buffer.push(b'\n');
                    offset += 31; // 每行 30 字符，1 个换行符
                }

                // 确保不超出目标范围
                let buffer_size = buffer.len();
                let chunk_slice = if buffer_size > end - start {
                    &buffer[..(end - start)]
                } else {
                    &buffer[..buffer_size]
                };

                // 通过锁定的映射文件写入数据块
                let mut mmap = mmap.lock().unwrap();
                mmap[start..start + chunk_slice.len()].copy_from_slice(chunk_slice);
            })
        })
        .collect();

    // 等待所有线程完成
    for handle in handles {
        handle.await.expect("Thread panicked");
    }

    let duration = start_time.elapsed(); // 計算執行時間

    println!("函數執行時間：{:?}", duration); // 輸出執行時間

    // let content_disposition = format!("attachment; filename={}", file_name);
    // let body = axum::body::Body::from(fs::read(&file_name).expect("Failed to read file"));
    let duration_message = format!("<p>函數執行時間：{:?}</p>", duration);
    let body_content = duration_message;

    let body = axum::body::Body::from(body_content);

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(body)
        .unwrap()
}
