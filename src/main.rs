use std::{fs::write, io};

use algos::{binary_tree::binary_tree, sidewinder::sidewinder};
use base_grid::Svg;
use djikstra::DjikstraGrid;
use grid::StandardGrid;
use rand::rngs::SmallRng;
use rand_seeder::Seeder;

mod algos;
mod base_grid;
mod cell;
mod distances;
mod djikstra;
mod grid;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn main() -> Result<(), io::Error> {
    let mut args = std::env::args();
    let seed = "abc12345abc";
    let mut rng: SmallRng = Seeder::from(&seed).into_rng();

    let mut grid = DjikstraGrid::new(8, 8);
    // let mut grid = StandardGrid::new(8, 8);

    let algo: &str = &args.nth(1).unwrap_or("binary".to_string());

    match algo {
        "sidewinder" => sidewinder(&mut grid.grid, &mut rng),
        _ => binary_tree(&mut grid.grid, &mut rng),
    };

    println!("{}", grid.grid);

    // let output = grid.draw();
    // write("./test.svg", output)?;

    Ok(())
}
