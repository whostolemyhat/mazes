use std::{collections::HashMap, vec};

use crate::Position;

pub fn distances(
    root: &Position,
    links: &HashMap<Position, Vec<Position>>,
) -> HashMap<Position, i32> {
    let mut distances = HashMap::new();
    distances.insert(*root, 0);
    let mut frontier = vec![root];

    while frontier.len() > 0 {
        let mut new_frontier = vec![];

        // TODO links are added in row/col order
        // so iterate over all cells and check
        for pos in frontier {
            // get vec from pos
            if let Some(cell_links) = links.get(&pos) {
                for link in cell_links {
                    if distances.contains_key(&link) {
                        continue;
                    }
                    distances.insert(*link, distances[&pos] + 1);
                    new_frontier.push(link);
                }
            }
        }
        frontier = new_frontier;
    }

    distances
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use rand::rngs::SmallRng;
    use rand_seeder::Seeder;

    use crate::{
        Position,
        algos::sidewinder::sidewinder,
        grid::{Grid, StandardGrid},
    };

    use super::distances;

    #[test]
    fn it_should_find_distances() {
        let mut grid: Box<dyn Grid> = Box::new(StandardGrid::new(4, 4));
        let seed = "abc12345abc";
        let mut rng: SmallRng = Seeder::from(&seed).into_rng();
        sidewinder(&mut grid, &mut rng);

        let distances = distances(&grid.map()[0].position, &grid.links());

        let expected = HashMap::from([
            (Position { x: 2, y: 1 }, 7),
            (Position { x: 0, y: 2 }, 10),
            (Position { x: 2, y: 3 }, 7),
            (Position { x: 1, y: 3 }, 8),
            (Position { x: 3, y: 2 }, 5),
            (Position { x: 2, y: 0 }, 2),
            (Position { x: 1, y: 0 }, 1),
            (Position { x: 3, y: 3 }, 6),
            (Position { x: 2, y: 2 }, 6),
            (Position { x: 1, y: 2 }, 9),
            (Position { x: 1, y: 1 }, 10),
            (Position { x: 0, y: 0 }, 0),
            (Position { x: 3, y: 1 }, 4),
            (Position { x: 3, y: 0 }, 3),
            (Position { x: 0, y: 3 }, 9),
            (Position { x: 0, y: 1 }, 11),
        ]);
        assert_eq!(distances, expected);
    }
}
