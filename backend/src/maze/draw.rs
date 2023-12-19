use std::collections::HashSet;
use svg::node::element::path::Data;
use svg::node::element::{Path, Rectangle, Mask, SVG};
use svg::Document;

use crate::maze::Wall::*;
use crate::maze::{Cell, Maze, Wall};

const CELL_SIDE: u32 = 10;
const STROKE_WIDTH: u32 = 2;

pub fn draw(maze: &Maze) -> Vec<Path> {
  let mut paths = vec![];

  for row in 0..maze.height() {
    for col in 0..maze.width() {
      let cell = (row, col);
      add_cell_paths(&mut paths, &maze, cell, &maze[cell]);
    }
  }
  //paths.push(make_line((10, 5),(10,0), "red"));
  return paths;
}

pub fn fill_maze(maze: &Maze) -> Vec<Rectangle> {
  let mut squares = vec![];
  for row in 0..maze.height() {
    for col in 0..maze.width() {
      let cell = (row, col);
        fill_cell(&mut squares, cell);
    }
  }
  return squares;
}

pub fn create_document(paths: &Vec<Path>, squares: Option<&Vec<Rectangle>>, maze: &Maze) -> Document {
  let (width, height) = (maze.width() * CELL_SIDE, maze.height() * CELL_SIDE);
  let mut document = Document::new()
    .set("viewBox", (0, 0, width, height))
    .set("style", "object-fit: fill;, width: 100%, height: 100%");
  if let Some(squares) = squares {
    document = document.add(masked_group(paths, squares));
  }
  else {
    document = paths.into_iter().fold( document, |doc, path| doc.add(path.clone()));
  }
  //println!("{}", document.clone().to_string());
  return document;
}

//TODO CREATE THE add_to_document FUNCTION THAT WILL TAKE THE PATHS/SQUARES AND ADD THEM TO AN EXISTING DOCUMENT

pub fn add_to_document(document: &mut Document, paths: &Vec<Path>, squares: Option<&Vec<Rectangle>>) -> Document {
  let mut newDocument = document.clone();
  if let Some(squares) = squares {
    newDocument = newDocument.add(masked_group(paths, squares));
  }
  newDocument
}


fn masked_group(paths: &Vec<Path>, squares: &Vec<Rectangle>) -> svg::node::element::Group {
  // Create a group to apply the mask to both the rectangle and the path line
  let mut group = svg::node::element::Group::new().set("mask", "url(#mask)");

  // Add rectangles to the group
  squares.clone().into_iter().for_each(|square| {
      group = group.clone().add(square);
  });

  // Add paths to the group
  paths.clone().into_iter().for_each(|path| {
      group = group.clone().add(path);
  });

  group
}


fn make_line(from: (u32, u32), relative_to: (u32, u32), color: &str, drawing_type: &str) -> Path {
  let data = Data::new().move_to(from).line_by(relative_to);

  Path::new()
    .set("fill", "none")
    .set("stroke", color)
    .set("stroke-width", STROKE_WIDTH)
    .set("stroke-linejoin", "square")
    .set("stroke-linecap", "square")
    .set("fill-opacity", "1")
    .set("d", data)
}


//This function is only to construct the maze, it will not be used afterwards.
fn add_cell_paths(paths: &mut Vec<Path>, maze: &Maze, (row, col): Cell, walls: &HashSet<Wall>) {
  let left_corner = (col * CELL_SIDE, row * CELL_SIDE);
  let (left_corner_x, left_corner_y) = left_corner;

  for &wall in walls {
    match wall {
      Top => {
        let path = make_line(left_corner, (CELL_SIDE, 0), "black", "line");
        paths.push(path)
      }
      Left => {
        let path = make_line(left_corner, (0, CELL_SIDE), "black", "line");
        paths.push(path)
      }
      // only draw right and bottom for right and bottom edges, to avoid double lines
      Right => {
        if col == maze.width() - 1 {
          let path = make_line((left_corner_x + CELL_SIDE, left_corner_y), (0, CELL_SIDE), "black", "line");
          paths.push(path);
        }
      }
      Bottom => {
        if row == maze.height() - 1 {
          let path = make_line((left_corner_x, left_corner_y + CELL_SIDE), (CELL_SIDE, 0), "black", "line");
          paths.push(path);
        }
      }
    };
  }
}

fn make_square(x: u32, y: u32, width: u32, height: u32) -> Rectangle {
  Rectangle::new()
    .set("x", x)
    .set("y", y)
    .set("width", width)
    .set("height",height)
    .set("fill", "grey")
    .set("fill-opacity", "0.5")
}


pub fn fill_cell(squares: &mut Vec<Rectangle>, (row, col): Cell) {
  squares.push(make_square(col * CELL_SIDE, row * CELL_SIDE, CELL_SIDE, CELL_SIDE));
}
