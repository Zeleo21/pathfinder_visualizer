use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, http};
use std::time::Instant;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;
use backend::draw::draw;
use backend::maze::Maze;
use backend::draw::create_document;


//Global variables for SVG
static mut PATHS: Vec<Path> = vec![];
static mut  DOCUMENT: Option<Document> = None;

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
      unsafe { 
        PATHS  = draw(&maze);
        //PATHS.clone().into_iter().for_each(|line| println!("{}", line));
        DOCUMENT = Some(create_document(PATHS.clone(), &maze));
        println!("Saved to SVG in {:?}.", t.elapsed());
        match &DOCUMENT { 
            Some(x) => return HttpResponse::Ok().body(x.to_string()),
            None => return HttpResponse::InternalServerError().body("Internal Server Error"),
        }
    };
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