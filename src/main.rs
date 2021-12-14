use std::env;

mod binary_tree;
mod cell;
mod grid;
mod sidewinder;

use binary_tree::BinaryTree;
use grid::Grid;
use sidewinder::Sidewinder;

// tODO fix display so 0,0 is southwest not northwest
// then update algos to use north not south

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    println!("pick an algorithm");
    return;
  }

  let mut grid = Grid::new(8, 18);
  // operates on grid in-place
  match args[1].as_str() {
    "binary" => {
      let _binary_tree = BinaryTree::on(&mut grid);
    }
    "sidewinder" => {
      let _sidewinder = Sidewinder::on(&mut grid);
    }
    _ => {
      println!("Thats not an algo");
      return;
    }
  }
  println!("{}", grid);
}
