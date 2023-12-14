use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, http};
use backend::draw::{draw, fill_maze};
use backend::maze::Maze;
use backend::draw::create_document;
use regex::Regex;
use svg::Document;
use svg::node::Value;
use svg::node::element::Path;
use svg::node::element::tag::Tag;
use svg::parser::Event;
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

async fn get_maze(data: web::Data<AppState>, req: web::Json<MazeRequest>) -> impl Responder {
      let params = req.into_inner();
      if params.width == 0 || params.height == 0 {
          return HttpResponse::BadRequest().body("Invalid request, please provide valid width and height");
      }
      let maze = Maze::generate(params.width, params.height);
      println!("Generated {}x{} maze", params.width, params.height);

      //We create the SVG structure
      let paths = draw(&maze);
      let document = create_document(&paths, None, &maze);

      //This is to save the maze into the app state for reusability.

      //
      //TODO: This part should save the current state of the maze into the app state.
      //      Create a new directory called /utils/serializers and create a new file called app_state.rs
      //      This file should handle the maze state to have persistent data accessibility with serializer and deserializer of the maze.
      //

      let mut app_state_paths = data.paths.lock().unwrap();
      *app_state_paths = paths.clone().into_iter().map(|path| { path.to_string() + "\n" }).collect::<String>();
      println!("App state is now \n {}", *app_state_paths);
      println!("------------------------------------------------------------------------");
      let newPaths = deserialize_path(&app_state_paths);

      let newDoc = create_document(&newPaths, None, &maze);
      println!("{} ", newDoc.to_string());

      //
      //
      //

      return HttpResponse::Ok().body(document.to_string());
}

pub fn deserialize_path(data: &String) -> Vec<Path> {
    let mut res = vec![];
    for path in data.split('\n') {
        if let Some(path) = extract_attributes(path) {
            res.push(Path::new().set("d", path));
        }
    }
    return res;
}


fn extract_attributes(svg_path: &str) -> Option<String> {
    // Regular expression to match the attributes inside the path tag
    let re = Regex::new(r#"<path([^>]+)/>"#).unwrap();

    // Extract the matched attributes
    if let Some(captures) = re.captures(svg_path) {
        if let Some(attributes) = captures.get(1) {
            return Some(attributes.as_str().trim().to_string());
        }
    }

    None
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