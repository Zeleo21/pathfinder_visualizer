use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, http};
use std::error::Error;
use std::time::Instant;

use backend::draw::draw;
use backend::maze::Maze;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(serde::Deserialize)]
struct MazeRequest {
    width: u32,
    height: u32
}

async fn get_maze(req: web::Json<MazeRequest>) -> impl Responder {
      let params = req.into_inner();
      if params.width == 0 || params.height == 0 {
          return HttpResponse::BadRequest().body("Invalid request, please provide valid width and height");
      }
      let maze = Maze::generate(params.width, params.height);
      println!("Generated {}x{} maze", params.width, params.height);

      let t = Instant::now();
      let document = draw(&maze);
      println!("Saved to SVG in {:?}.", t.elapsed());
      return HttpResponse::Ok().body(document.to_string());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
      HttpServer::new(|| {
        let cors = actix_cors::Cors::default().allow_any_origin()
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE);
        App::new()
            .wrap(cors)
            .service(hello)
            .route("/maze", web::post().to(get_maze))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}