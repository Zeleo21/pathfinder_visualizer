use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use std::error::Error;
use std::time::Instant;

use backend::draw::draw;
use backend::maze::Maze;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
 
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/maze")]
async fn get_maze(req: HttpRequest) -> impl Responder {
      let maze = Maze::generate(3, 3);
      println!("Generated {}x{} maze", 3, 3);

      let t = Instant::now();
      let document = draw(&maze);
      let _ = svg::save("image.svg", &document);
      println!("Saved to SVG in {:?}.", t.elapsed());
      if let Ok(svg_content) = web::block(|| std::fs::read_to_string("image.svg")).await {
          return HttpResponse::Ok().body(svg_content.unwrap());
      }else {
          return HttpResponse::InternalServerError().body("Internal Server Error");
      }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
      HttpServer::new(|| {
        let cors = actix_cors::Cors::default().allow_any_origin();
        App::new()
            .wrap(cors)
            .service(hello)
            .service(echo)
            .service(get_maze)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}