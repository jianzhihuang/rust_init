// src/main.rs
use warp::Filter;

#[tokio::main]
async fn main() {
    // 創建路由
    let hello = warp::path!("hello" / "world").map(|| warp::reply::html("你好，世界！"));

    // 啟動服務器
    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
