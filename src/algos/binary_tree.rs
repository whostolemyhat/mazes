use rand::rngs::SmallRng;
use rand::seq::IndexedRandom;

use crate::{Direction, grid::Grid};

// carve south or east, starting at top left
pub fn binary_tree(grid: &mut dyn Grid, rng: &mut SmallRng) {
    let mut links = grid.links().clone();
    for cell in grid.map() {
        let mut neighbours = vec![];
        if cell.neighbours.contains_key(&Direction::South) {
            neighbours.push(
                *cell
                    .neighbours
                    .get(&Direction::South)
                    .expect("Couldn't get south neighbour"),
            );
        }
        if cell.neighbours.contains_key(&Direction::East) {
            neighbours.push(
                *cell
                    .neighbours
                    .get(&Direction::East)
                    .expect("Couldn't get east neighbour"),
            );
        }

        if !neighbours.is_empty() {
            let neighbour = neighbours.choose(rng).expect("Couldn't pick neighbour");

            links
                .entry(cell.position)
                .or_insert(vec![])
                .push(*neighbour);
            // grid.link(&cell.position, neighbour);
            links
                .entry(*neighbour)
                .or_insert(vec![])
                .push(cell.position);
        }
    }

    grid.set_links(links);
}

#[cfg(test)]
mod test {
    use rand::rngs::SmallRng;
    use rand_seeder::Seeder;

    use crate::{algos::binary_tree::binary_tree, grid::StandardGrid};

    #[test]
    fn should_generate_maze() {
        let seed = "abc12345abc";
        let mut rng: SmallRng = Seeder::from(&seed).into_rng();
        let mut container = StandardGrid::new(4, 4);
        binary_tree(&mut container, &mut rng);

        assert_eq!(
            format!("{}", container),
            "+---+---+---+---+
|               |
+---+---+---+   +
|   |       |   |
+   +---+   +   +
|   |       |   |
+   +---+   +   +
|               |
+---+---+---+---+
"
        );
    }
}
