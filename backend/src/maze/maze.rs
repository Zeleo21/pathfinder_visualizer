use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use Wall::*;

/// (row, col), 0-indexed
pub type Cell = (u32, u32);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Wall {
  Top,
  Right,
  Bottom,
  Left,
}

impl Wall {
  fn opposite(self) -> Wall {
    match self {
      Top => Bottom,
      Bottom => Top,
      Left => Right,
      Right => Left,
    }
  }
}

impl std::fmt::Display for Wall {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Top => write!(f, "Top"),
      Right => write!(f, "Right"),
      Bottom => write!(f, "Bottom"),
      Left => write!(f, "Left"),
    }
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Maze {
  width: u32,
  height: u32,
  walls: HashMap<Cell, HashSet<Wall>>,
  current_cell: Cell,
}
impl Maze {
  pub fn new_empty() -> Maze {
    let walls = HashMap::new();
    Maze { width: 0, height: 0, walls, current_cell: (0, 0) }
  }
  pub fn new(width: u32, height: u32) -> Maze {
    let walls = HashMap::new();
    Maze { width, height, walls, current_cell: (0, 0) }
  }

  pub fn get_current_cell(&mut self) -> Cell {
    self.current_cell
  }
  
  pub fn set_current_cell(&mut self, cell: Cell) {
    self.current_cell = cell;
  }
  
  pub fn add_cell(&mut self, cell: Cell, walls: &[Wall]) -> &mut Maze {
    self.walls.insert(cell, walls.into_iter().copied().collect());
    self
  }

  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
  }

  pub fn neighbour(&self, cell: Cell, wall: Wall) -> Option<Cell> {
    let (row, col) = cell;
    match wall {
      Top => {
        if row > 0 {
          Some((row - 1, col))
        } else {
          None
        }
      }
      Bottom => {
        if row < self.height() - 1 {
          Some((row + 1, col))
        } else {
          None
        }
      }
      Left => {
        if col > 0 {
          Some((row, col - 1))
        } else {
          None
        }
      }
      Right => {
        if col < self.width() - 1 {
          Some((row, col + 1))
        } else {
          None
        }
      }
    }
  }
  
  pub fn free_neighbour(&self, cell: Cell) -> HashSet<Option<Cell>> {
    let all_walls: HashSet<Wall> = [Top, Right, Bottom, Left].iter().cloned().collect();
    let binding = HashSet::new();
    let cell_walls = self.walls.get(&cell).unwrap_or(&binding);
    let difference: HashSet<Wall> = all_walls.difference(cell_walls).cloned().collect();
    let mut free_cells: HashSet<Option<Cell>>   = HashSet::new();
    difference.into_iter().for_each(|wall| {
      if let Some(neighbour) = self.neighbour(cell, wall) {
        println!("the cell at : {} {} has a neighbour in the {} direction", cell.0, cell.1, wall);
        free_cells.insert(Some(neighbour));
      }
    });
    return free_cells;
  }

  pub fn get_possible_directions(&self, cell: Cell) -> HashSet<Wall> {
    let all_walls: HashSet<Wall> = [Top, Right, Bottom, Left].iter().cloned().collect();
    let binding = HashSet::new();
    let cell_walls = self.walls.get(&cell).unwrap_or(&binding);
    return all_walls.difference(cell_walls).cloned().collect();
  }
}

impl std::ops::Index<Cell> for Maze {
  type Output = HashSet<Wall>;

  fn index(&self, index: Cell) -> &Self::Output {
    self
      .walls
      .get(&index)
      .expect(&format!("Cell at {:?} doesn't exist.", &index))
  }
}

impl Maze {
  /// Generates a Maze with `width` and `height`
  /// using Prim's algorithm.
  pub fn generate(width: u32, height: u32) -> Maze {
    let mut maze = Maze::new(width, height);

    // surround each cell with walls
    let all_walls = &[Top, Right, Bottom, Left];
    for row in 0..height {
      for col in 0..width {
        maze.add_cell((row, col), all_walls);
      }
    }

    let start_cell = (0, 0);
    let mut in_maze = HashSet::new();
    in_maze.insert(start_cell);
    let mut walls = vec![];
    maze.add_cell_walls_to_vec(&mut walls, start_cell);

    while let Some((cell, wall)) = walls.pop() {
      if let Some(neighbour) = maze.neighbour(cell, wall) {
        if !in_maze.contains(&neighbour) {
          in_maze.insert(neighbour);
          maze.add_cell_walls_to_vec(&mut walls, neighbour);

          maze.remove_wall(cell, wall);
        }
      }
    }

    maze
  }

  fn add_cell_walls_to_vec(&self, walls: &mut Vec<(Cell, Wall)>, cell: Cell) {
    for &wall in &self[cell] {
      walls.push((cell, wall));
    }
  }

  /// Removes `wall` from `cell`, and the corresponding wall
  /// of the `cell`'s neighbour, if it exists.
  fn remove_wall(&mut self, cell: Cell, wall: Wall) {
    match self.walls.get_mut(&cell) {
      Some(walls) => {
        walls.remove(&wall);

        if let Some(neighbour) = self.neighbour(cell, wall) {
          if let Some(neighbour_walls) = self.walls.get_mut(&neighbour) {
            neighbour_walls.remove(&wall.opposite());
          }
        }
      }
      None => (),
    }
  }
}

