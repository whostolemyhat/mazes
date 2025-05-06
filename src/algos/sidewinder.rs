use std::collections::HashMap;

use crate::grid::Grid;
use crate::{Direction, Position};
use rand::rngs::SmallRng;
use rand::{Rng, seq::IndexedMutRandom};

fn link(links: &mut HashMap<Position, Vec<Position>>, start: &Position, neighbour: &Position) {
    links.entry(*start).or_insert(vec![]).push(*neighbour);
}

pub fn sidewinder(grid: &mut dyn Grid, rng: &mut SmallRng) {
    // can't borrow links field inside for loop since Rust doesn't know which field
    // we're mutating
    // so take links here, update in loop, then reassign
    // https://stackoverflow.com/a/64921799
    let mut links = grid.links().clone();

    grid.map().iter().for_each(|cell| {
        let mut run = vec![];
        let cell_clone = cell.clone();
        let eastern_neighbour = cell_clone.neighbours.get(&Direction::East);
        let at_eastern_boundary = eastern_neighbour.is_none();
        let at_southern_boundary = !cell_clone.neighbours.contains_key(&Direction::South);

        let close_run = at_eastern_boundary || (!at_southern_boundary && rng.random_bool(0.5));
        run.push(cell);

        if close_run {
            let chosen = run.choose_mut(rng).expect("Failed to pick from run");
            let southern_neighbour = chosen.neighbours.get(&Direction::South);
            if let Some(southern_neighbour) = southern_neighbour {
                // grid.link(&chosen.position, &southern_neighbour.clone());
                link(&mut links, &chosen.position, &southern_neighbour.clone());
            }

            run.clear();
        } else if eastern_neighbour.is_some() {
            let index = run.len() - 1;
            link(
                &mut links,
                &run[index].position,
                eastern_neighbour.expect("Couldn't get south pos"),
            );
        }
    });

    grid.set_links(links);
}

#[cfg(test)]
mod test {
    use rand::rngs::SmallRng;
    use rand_seeder::Seeder;

    use crate::{algos::sidewinder::sidewinder, grid::StandardGrid};

    #[test]
    fn should_generate_maze() {
        let seed = "abc12345abc";
        let mut rng: SmallRng = Seeder::from(&seed).into_rng();
        let mut container = StandardGrid::new(4, 4);
        sidewinder(&mut container, &mut rng);

        assert_eq!(
            format!("{}", container),
            "+---+---+---+---+
|               |
+---+---+---+   +
|   |   |   |   |
+   +   +   +   +
|   |   |       |
+   +   +---+   +
|               |
+---+---+---+---+
"
        );
    }
}
