use std::f32::consts::PI;

use actix_web::{get, web, App, HttpServer, Responder};

// 定义处理器函数
#[get("/")]
async fn index() -> impl Responder {
    "Hello, Rust!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动HTTP服务器并绑定到本地地址的8080端口
    HttpServer::new(|| {
        App::new()
            // 注册路由
            .service(index)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}


#[test]
fn test_print_pi() {
    println!("this is pi {}", PI);
}