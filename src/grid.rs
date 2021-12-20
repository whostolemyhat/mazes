use crate::cell::Cell;
use cairo::{Context, Format, ImageSurface};
use rand::prelude::*;
use std::fmt::{self, Display, Formatter};
use std::fs::File;

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

  pub fn to_image(&self, cell_size: i32) {
    let img_width = cell_size * self.columns;
    let img_height = cell_size * self.rows;

    let surface = ImageSurface::create(Format::ARgb32, img_width + 20, img_height + 20)
      .expect("Couldn't create surface");
    let context = Context::new(&surface).expect("Failed to create context");
    let offset: f64 = 10.0;
    // background
    context.set_source_rgb(1.0, 1.0, 1.0);
    context.rectangle(
      0.,
      0.,
      img_width as f64 + (offset * 2.0),
      img_height as f64 + (offset * 2.0),
    );
    context.fill().ok();

    for cell in &self.map {
      let x1: f64 = (cell.column * cell_size) as f64 + offset;
      let x2: f64 = ((cell.column + 1) * cell_size) as f64 + offset;
      let y1: f64 = (cell.row * cell_size) as f64 + offset;
      let y2: f64 = ((cell.row + 1) * cell_size) as f64 + offset;

      let cell_pos = cell.position();

      context.new_path();
      context.move_to(x1, y1);

      if cell.neighbours.north.is_some() {
        context.move_to(x2, y1);
      } else {
        context.line_to(x2, y1);
      }
      if cell.is_linked((cell_pos.0, cell_pos.1 + 1)) {
        context.move_to(x2, y2);
      } else {
        context.line_to(x2, y2);
      }
      if cell.is_linked((cell_pos.0 + 1, cell_pos.1)) {
        context.move_to(x1, y2)
      } else {
        context.line_to(x1, y2);
      }

      // only draw west for outer walls
      if cell.neighbours.west.is_some() {
        context.move_to(x1, y1);
      } else {
        context.line_to(x1, y1);
      }

      let line_colour = (0.1, 0.1, 0.0);
      context.set_line_width(2.0);
      context.set_source_rgb(line_colour.0, line_colour.1, line_colour.1);
      context.stroke().expect("Failed to draw");
    }

    let mut file = File::create("maze.png").expect("Can't create file for some reason");
    surface
      .write_to_png(&mut file)
      .expect("Failed to draw image");
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
        let index = ((row * self.columns) + col) as usize;
        let cell = self.map[index].clone();
        top = top + "   ";
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
      output = output + &format!("{}\n{}\n", &top, &bottom).to_owned();
    }
    write!(f, "{}", output)
  }
}
