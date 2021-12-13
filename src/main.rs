use rand::prelude::*;

mod cell;
mod grid;

use grid::Grid;

struct BinaryTree {}

impl BinaryTree {
  fn on(grid: &mut Grid) {
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

fn main() {
  let mut grid = Grid::new(4, 4);
  // println!("{:?}", grid);
  // currently operates on grid in-place
  let binary_tree = BinaryTree::on(&mut grid);
  println!("{}", grid);
  // println!("{:?}", grid);
}
