use crate::{Direction, grid::Grid};
use rand::rngs::SmallRng;
use rand::{Rng, seq::IndexedMutRandom};

pub fn sidewinder(grid: &mut Grid, rng: &mut SmallRng) {
    grid.map.iter_mut().for_each(|cell| {
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
                chosen.link(&southern_neighbour.clone());
            }

            run.clear();
        } else if eastern_neighbour.is_some() {
            let index = run.len() - 1;
            run[index].link(eastern_neighbour.expect("Couldn't get south pos"));
        }
    });
}

#[cfg(test)]
mod test {
    use rand::rngs::SmallRng;
    use rand_seeder::Seeder;

    use crate::{algos::sidewinder::sidewinder, grid::Grid};

    #[test]
    fn should_generate_maze() {
        let seed = "abc12345abc";
        let mut rng: SmallRng = Seeder::from(&seed).into_rng();
        let mut grid = Grid::new(4, 4);
        sidewinder(&mut grid, &mut rng);

        assert_eq!(
            format!("{}", grid),
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
