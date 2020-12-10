mod spidar_mouse;
use spidar_mouse::{post::impl_post,ws::impl_ws};

use actix_web::{App, HttpResponse, HttpServer, Responder, web, get};

const INDEX: &str = include_str!("./static/index.html");
const WS_INDEX: &str = include_str!("./static/ws.html");
const WSJS: &str = include_str!("./static/ws.js");

#[get("/")]
async fn index() -> impl Responder{
    HttpResponse::Ok().body(INDEX)
}

#[get("/ws.html")]
async fn ws_index() -> impl Responder {
    HttpResponse::Ok().body(WS_INDEX)
}

#[get("/ws.js")]
async fn ws_js() -> impl Responder{
    HttpResponse::Ok().body(WSJS)
}

// #[get("open")]
// async fn mouse() -> impl Responder {
//     HttpResponse::Ok().body(format!("{}", open_spidar_mouse()))
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("listen http://localhost:8080/");
    HttpServer::new(|| {
        App::new()
        .service(web::resource("/ws/").to(impl_ws))
        .route("/api/", web::post().to(impl_post))
        .service(index)
        .service(ws_index)
        .service(ws_js)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}