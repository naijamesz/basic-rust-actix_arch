use actix_web:: HttpServer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        actix_web::App::new()
            .route("/", actix_web::web::get().to(index))
            .service(
                actix_web::web::scope("/v1")
                    .route("/profile", actix_web::web::get().to(index))
                    .route("/profile", actix_web::web::post().to(insert))
            )
            .service(
                actix_web::web::scope("/v2")
                    .route("/profile", actix_web::web::get().to(index))
                    .route("/admin", actix_web::web::post().to(insert))
            )
    })
    .bind(("127.0.0.1", 8001))?
    .run()
    .await
}


async fn index() -> &'static str{
    "Hello from index"
}
async fn insert() -> &'static str{
    "Inserted"
}


// #[get("/")]
// async fn hello() -> impl Responder{
//     HttpResponse::Ok().body("Hello World!")
// }


// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello()-> impl Responder{
//     HttpResponse::Ok().body("Hello! There")
// }

// #[patch("/echo")]
// async fn patch() -> impl Responder{
//     HttpResponse::Ok().body("Patched")
// }