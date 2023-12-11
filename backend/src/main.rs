// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::error::Error;
use std::time::Instant;

use backend::draw::draw;
use backend::maze::Maze;


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

fn main() -> Result<(), Box<dyn Error>> {
  let args = std::env::args()
    .skip(1)
    .map(|arg| arg.parse::<u32>())
    .collect::<Result<Vec<_>, _>>()?;

  if args.len() >= 2 {
    let (width, height) = (args[0], args[1]);

    let t = Instant::now();
    let maze = Maze::generate(width, height);
    println!("Generated {}x{} maze in {:?}.", width, height, t.elapsed());

    let t = Instant::now();
    let document = draw(&maze);
    svg::save("image.svg", &document)?;
    println!("Saved to SVG in {:?}.", t.elapsed());

    Ok(())
  } else {
    Err(format!("Invalid args (expected width and height): {:?}", args).into())
  }
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