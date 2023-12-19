use actix::{Actor, StreamHandler};
use actix_web::{web, error::InternalError};
use actix_web_actors::ws;
use backend::{maze::Maze, draw::{draw, fill_maze, create_document, fill_cell, add_to_document}};
use svg::Document;
use std::{time::{Duration, Instant}, sync::Mutex};

use actix::prelude::*;

use crate::MazeRequest;
/// Define HTTP actor

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Clone)]
pub struct MyWs {
    hb: Instant,
    maze: Maze,
    active_mode: bool,
    document: Document,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
        /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

impl MyWs {
    pub fn new(maze: Maze, document: Document) -> Self {
        Self { hb: Instant::now(), maze: maze , active_mode: true, document: document}
    }

    pub fn update_maze(&mut self, maze: Maze) {
        self.maze = maze;
    }
    pub fn update_document(&mut self, document: Document) {
        self.document = document;
    }
    /// helper method that sends ping to client every 5 seconds (HEARTBEAT_INTERVAL).
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }
            println!("Websocket Client heartbeat ok");
            ctx.ping(b"Connection alive");
        });
    }
    

    fn dfs(&mut self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(Duration::from_millis(100), |act, ctx| {
            if !act.active_mode {
                return;
            }
            println!("dfs");
            let paths = draw(&act.maze);
            let mut squares = vec![];
            let current_cell = act.maze.get_current_cell();

            if current_cell == (act.maze.width() - 1, act.maze.height() - 1) {
                println!("--------------FOUND THE EXIT SIUUUUUU------------");
                return;
            }

            fill_cell(&mut squares, current_cell);
            let free_neighbors = act.maze.free_neighbour(current_cell);
            println!("Free neighbors: {:?}", free_neighbors);
            act.maze.set_current_cell(free_neighbors.into_iter().nth(0).unwrap().unwrap());

            let new_document = add_to_document(&mut act.document, &paths, Some(&squares));
            act.update_document(new_document);

            ctx.text(act.document.to_string());
        });
    }

    fn set_active_mode(&mut self) {
        self.active_mode = !self.active_mode;
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                if text =="dfs" {
                    self.dfs(ctx);
                }
                else if text == "stop" {
                    self.set_active_mode();
                }
                else  {
                    ctx.text(text);
                }
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                println!("Websocket Client disconnected: {:?}", reason);
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}