use rand::prelude::*;

use crate::grid::Grid;

pub struct BinaryTree {}

impl BinaryTree {
  pub fn on(grid: &mut Grid) {
    for cell in grid.map.iter_mut() {
      let mut neighbours = vec![];
      if cell.neighbours.south.is_some() {
        neighbours.push(cell.neighbours.south);
      }
      if cell.neighbours.east.is_some() {
        neighbours.push(cell.neighbours.east);
      }

      let mut rng = rand::thread_rng();
      let neighbour = neighbours.choose(&mut rng);

      // check there are actually neighbours
      if let Some(neighbour) = neighbour {
        cell.link(&neighbour.expect("Couldn't get cell neighbour"));
      }
    }
  }
}
