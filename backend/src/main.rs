// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod maze;

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }
 
// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

fn main() {
    maze::mazegenerator::visualize_maze();
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     maze::mazegenerator::maze_generator();
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(echo)
//             .route("/hey", web::get().to(manual_hello))
//     })
//     .bind(("127.0.0.1", 7070))?
//     .run()
//     .await
// }