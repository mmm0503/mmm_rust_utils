use actix_web::{App, get, HttpServer, Responder};

// 异步服务
#[actix_web::main]
pub async fn open_server() {
    let _server = HttpServer::new(|| {
        App::new()
            .service(index)
    })
        .bind("127.0.0.1:3000").unwrap()
        .run()
        .await;
}

#[get("/send")]
async fn index() -> impl Responder {
    "Hello world!"
}
