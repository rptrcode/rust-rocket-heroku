use actix_web::{App, HttpResponse, HttpServer, Responder, web};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .route("/rptr", web::get().to(rptr))
    })
        .bind("0.0.0.0:3000")?
        .run()
        .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn rptr() -> impl Responder {
    format!("rptr");
    HttpResponse::Ok().body("Hello rptr!")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}
