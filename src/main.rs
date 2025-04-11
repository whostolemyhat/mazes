use grid::Grid;
use rand::{Rng, seq::IndexedRandom};

mod cell;
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

// carve south or east, starting at top left
fn binary_tree(grid: &mut Grid) {
    let mut rng = rand::rng();

    grid.map.iter_mut().for_each(|cell| {
        let mut neighbours = vec![];
        if cell.neighbours.get(&Direction::South).is_some() {
            neighbours.push(
                cell.neighbours
                    .get(&Direction::South)
                    .expect("Couldn't get south neighbour")
                    .clone(),
            );
        }
        if cell.neighbours.get(&Direction::East).is_some() {
            neighbours.push(
                cell.neighbours
                    .get(&Direction::East)
                    .expect("Couldn't get east neighbour")
                    .clone(),
            );
        }

        if neighbours.len() > 0 {
            let neighbour = neighbours
                .choose(&mut rng)
                .expect("Couldn't pick neighbour");

            cell.link(&neighbour);
        }
    });

    // // start bottom left
    // let start = Position {
    //     x: grid.width - 1,
    //     y: grid.height - 1,
    // };

    // let direction = if rng.random_bool(0.5) {
    //     Direction::North
    // } else {
    //     Direction::East
    // };
}

fn main() {
    let mut grid = Grid::new(4, 4);
    binary_tree(&mut grid);
    println!("{}", grid);
}
