use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, http};
use std::time::Instant;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;
use backend::draw::draw;
use backend::maze::Maze;
use backend::draw::create_document;
use std::sync::Mutex;

struct AppState {
    paths: Mutex<String>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(serde::Deserialize)]
struct MazeRequest {
    width: u32,
    height: u32
}

async fn get_maze(data: web::Data<AppState>,req: web::Json<MazeRequest>) -> impl Responder {
      let params = req.into_inner();
      if params.width == 0 || params.height == 0 {
          return HttpResponse::BadRequest().body("Invalid request, please provide valid width and height");
      }
      let maze = Maze::generate(params.width, params.height);
      println!("Generated {}x{} maze", params.width, params.height);

      //We create the SVG structure
      let paths = draw(&maze);
      let document = create_document(paths, &maze);

      //This is to save the maze into the app state for reusability.
      let mut app_state_paths = data.paths.lock().unwrap();
      *app_state_paths = document.to_string();
      println!("App state is now \n {}", *app_state_paths);
      
      return HttpResponse::Ok().body(document.to_string());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        paths: Mutex::new(String::new()),
    });
      HttpServer::new(move || {
        let cors = actix_cors::Cors::default().allow_any_origin()
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE);
        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .service(hello)
            .route("/maze", web::post().to(get_maze))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}