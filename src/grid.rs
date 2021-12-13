use rand::prelude::*;
use std::fmt::{self, Display, Formatter};

use crate::cell::Cell;

#[derive(Debug)]
pub struct Grid {
  rows: i32,
  columns: i32,
  pub map: Vec<Cell>,
}

impl Grid {
  pub fn new(rows: i32, columns: i32) -> Self {
    let mut map = Grid::prepare_grid(rows, columns);
    Grid::configure_cells(&mut map, rows, columns);
    let grid = Grid { rows, columns, map };
    grid
  }

  fn prepare_grid(rows: i32, columns: i32) -> Vec<Cell> {
    let mut grid = vec![];
    for i in 0..rows {
      for j in 0..columns {
        grid.push(Cell::new(i, j));
      }
    }

    grid
  }

  fn configure_cells(map: &mut Vec<Cell>, rows: i32, columns: i32) {
    for cell in map.iter_mut() {
      let row = cell.row;
      let column = cell.column;

      if row > 0 {
        cell.neighbours.north = Some((row - 1, column));
      }

      // tODO check this
      if row < rows - 1 {
        cell.neighbours.south = Some((row + 1, column));
      }

      if column < columns - 1 {
        cell.neighbours.east = Some((row, column + 1));
      }

      if column > 0 {
        cell.neighbours.west = Some((row, column - 1));
      }
    }
  }

  pub fn random_cell(&self) -> Option<&Cell> {
    let mut rng = rand::thread_rng();
    self.map.choose(&mut rng)
  }

  fn size(&self) -> i32 {
    self.rows * self.columns
  }
}

impl Display for Grid {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let mut output: String = "+".to_owned();
    for _ in 0..self.columns {
      output = output + "---+"
    }
    output = output + "\n";

    for row in 0..self.rows {
      let mut top = "|".to_owned();
      let mut bottom = "+".to_owned();

      for col in 0..self.columns {
        let cell = self.map[((row * self.rows) + col) as usize].clone();
        top = top + "   ";
        // let coords = format!("{},{}", row, col);
        // top = top + &coords.to_owned();
        let east = cell.links.get(&(row, col + 1));
        match east {
          Some(_) => top = top + " ",
          None => top += "|",
        }
        let south = cell.links.get(&(row + 1, col));
        match south {
          Some(_) => bottom = bottom + "   +",
          None => bottom = bottom + "---+",
        }
      }
      output = output + &top + "\n" + &bottom + "\n";
    }
    write!(f, "{}", output)
  }
}
