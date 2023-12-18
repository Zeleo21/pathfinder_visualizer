use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, http, middleware};
use backend::draw::{draw, fill_maze};
use backend::maze::Maze;
use backend::draw::create_document;
use regex::Regex;
use server::websocket::{MyWs};
use svg::Document;
use svg::node::Value;
use svg::node::element::Path;
use svg::node::element::tag::Tag;
use svg::parser::Event;
use std::sync::Mutex;
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;


mod server;



struct AppState {
    maze: Mutex<Maze>,
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

async fn dfs(data: web::Data<AppState>, req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    let resp  = ws::start(
        MyWs::new(),
        &req,
        stream
    );
    println!("{:?}", resp);
    resp
    // let maze = data.maze.lock().unwrap();
    // let paths = draw(&maze);
    // let squares = fill_maze(&maze);
    // let document = create_document(&paths, Some(&squares), &maze);
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

      let mut app_state_maze = data.maze.lock().unwrap();
      *app_state_maze = maze.clone();
      //*app_state_paths = paths.clone().into_iter().map(|path| { path.to_string() + "\n" }).collect::<String>();
     // println!("App state is now \n {}", *app_state_paths);
      println!("------------------------------------------------------------------------");
      //let newPaths = deserialize_path(&app_state_paths);

      //let newDoc = create_document(&newPaths, None, &maze);
      //println!("{} ", newDoc.to_string());

      //
      //
      //

      return HttpResponse::Ok().body(document.to_string());
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        maze: Mutex::new(Maze::new_empty()),
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
            .service(web::resource("/maze/dfs").route(web::get().to(dfs)))
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}