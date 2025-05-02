use std::{collections::HashMap, vec};

use crate::{Position, grid::Grid};

pub fn distances(root: &Position, grid: &Grid) -> HashMap<Position, i32> {
    let mut distances = HashMap::new();
    distances.insert(*root, 0);
    let mut frontier = vec![root];

    while frontier.len() > 0 {
        let mut new_frontier = vec![];

        for pos in frontier {
            // get vec from pos
            if let Some(cell_links) = grid.links.get(&pos) {
                for link in cell_links {
                    if distances.contains_key(&link) {
                        break;
                    }
                    dbg!(&distances, link, pos);
                    distances.insert(*link, distances[&pos] + 1);
                    new_frontier.push(link);
                }
            }
        }
        frontier = new_frontier;
    }

    // while frontier.len() > 0 {
    //     let mut new_frontier = vec![];

    //     for cell in frontier {
    //         for link in cell.links.iter() {
    //             if distances.contains_key(&link) {
    //                 break;
    //             }
    //             distances.insert(*link, distances[&cell.position] + 1);
    //             if let Some(linked_cell) = grid.cell_at(link) {
    //                 new_frontier.push(linked_cell);
    //             }
    //         }
    //     }

    //     frontier = new_frontier;
    // }

    distances
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use rand::rngs::SmallRng;
    use rand_seeder::Seeder;

    use crate::{Position, algos::sidewinder::sidewinder, grid::StandardGrid};

    use super::distances;

    #[test]
    fn it_should_find_distances() {
        let mut grid = StandardGrid::new(4, 4);
        let seed = "abc12345abc";
        let mut rng: SmallRng = Seeder::from(&seed).into_rng();
        sidewinder(&mut grid.grid, &mut rng);

        let distances = distances(&grid.grid.map[0].position, &grid.grid);

        let expected = HashMap::from([
            (Position { x: 3, y: 0 }, 3),
            (Position { x: 3, y: 2 }, 5),
            (Position { x: 3, y: 3 }, 6),
            (Position { x: 0, y: 0 }, 0),
            (Position { x: 3, y: 1 }, 4),
            (Position { x: 1, y: 0 }, 1),
            (Position { x: 2, y: 0 }, 2),
        ]);
        assert_eq!(distances, expected);
    }
}
