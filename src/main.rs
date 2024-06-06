use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn hello(req: HttpRequest) -> impl Responder {
    if let Some(peer_addr) = req.peer_addr() {
        println!("IP of the caller: {}", peer_addr.ip());
    }

    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(hello)))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
