use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

use rand::Rng;

use crate::{
    Position,
    base_grid::{GridSetup, Svg},
    cell::Cell,
};

#[derive(Debug, Eq, PartialEq)]
pub struct StandardGrid {
    pub map: Vec<Cell>,
    pub width: i32,
    pub height: i32,
    pub links: HashMap<Position, Vec<Position>>,
}

impl StandardGrid {
    pub fn new(width: i32, height: i32) -> Self {
        let mut map = Self::prepare_map(width, height);
        Self::configure_cells(&mut map, width, height);
        let links = HashMap::new();
        StandardGrid {
            map,
            width,
            height,
            links,
        }
    }
}

impl Grid for StandardGrid {
    fn map(&self) -> &Vec<Cell> {
        &self.map
    }
    fn width(&self) -> i32 {
        self.width
    }
    fn height(&self) -> i32 {
        self.height
    }
    fn links(&self) -> &HashMap<Position, Vec<Position>> {
        &self.links
    }
    fn links_mut(&mut self) -> &mut HashMap<Position, Vec<Position>> {
        &mut self.links
    }
    fn set_links(&mut self, links: HashMap<Position, Vec<Position>>) {
        self.links = links;
    }
}

pub trait Grid {
    fn map(&self) -> &Vec<Cell>;
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn links(&self) -> &HashMap<Position, Vec<Position>>;
    fn links_mut(&mut self) -> &mut HashMap<Position, Vec<Position>>;
    fn set_links(&mut self, links: HashMap<Position, Vec<Position>>);

    // pub fn new(width: i32, height: i32) -> Self {
    // let mut map = Grid::prepare_map(width, height);
    // Grid::configure_cells(&mut map, width, height);
    //     let links = HashMap::new();
    //     Grid {
    //         map,
    //         width,
    //         height,
    //         links: RefCell::new(links),
    //     }
    // }

    // pub fn new_with_djikstra(width: i32, height: i32) -> Self {
    //     let mut map = Grid::prepare_map(width, height);
    //     Grid::configure_cells(&mut map, width, height);
    //     let links = HashMap::new();
    //     Grid {
    //         map,
    //         width,
    //         height,
    //         links: RefCell::new(links),
    //     }
    // }

    fn random_cell(&self) -> Cell {
        let mut rng = rand::rng();
        let y = rng.random_range(0..self.height());
        let x = rng.random_range(0..self.width());

        self.map()[((y * self.width()) + x) as usize].clone()
    }

    fn cell_at(&mut self, pos: &Position) -> Option<&Cell> {
        self.map().iter().find(|cell| cell.position == *pos)
    }

    fn link(&mut self, start: &Position, neighbour: &Position) {
        self.links_mut()
            .entry(*start)
            .or_insert(vec![])
            .push(*neighbour);
    }

    fn is_linked(&self, start: &Position, pos: &Position) -> bool {
        match self.links().get(start) {
            Some(neighbours) => neighbours.contains(pos),
            _ => false,
        }
    }

    fn size(&self) -> i32 {
        self.width() * self.height()
    }
}

// impl BaseGrid for Grid {
//     fn width(&self) -> i32 {
//         self.width
//     }
//     fn height(&self) -> i32 {
//         self.height
//     }
//     fn map(&self) -> &Vec<Cell> {
//         &self.map
//     }
//     fn map_mut(&mut self) -> &mut Vec<Cell> {
//         &mut self.map
//     }
//     fn links(&mut self) -> &mut RefCell<HashMap<Position, Vec<Position>>> {
//         &mut self.links
//     }
// }

impl Svg for StandardGrid {}

impl GridSetup for StandardGrid {}

impl Display for StandardGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut output: String = "+".to_owned();
        for _ in 0..self.width() {
            output += "---+"
        }
        output += "\n";

        for y in 0..self.height() {
            let mut left = "|".to_owned();
            let mut bottom = "+".to_owned();

            for x in 0..self.width() {
                let index = ((y * self.width()) + x) as usize;
                let cell = self.map()[index].clone();
                let body = self.contents_of(&cell);
                left += &format!(" {} ", body);
                if let Some(cell_links) = self.links().get(&Position { x, y }) {
                    // east
                    if cell_links.contains(&Position { x: x + 1, y }) {
                        left += " ";
                    } else {
                        left += "|";
                    }
                    // match east {
                    // Some(_) => left += " ",
                    // None => left += "|",
                    // }

                    // south
                    if cell_links.contains(&Position { x, y: y + 1 }) {
                        bottom += "   +";
                    } else {
                        bottom += "---+"
                    }
                // match south {
                // Some(_) => bottom += "   +",
                // None => bottom += "---+",
                // }
                } else {
                    left += "|";
                    bottom += "---+"
                }

                // let south = self.links.get(&Position { x, y: y + 1 });
                // match south {
                //     Some(_) => bottom += "   +",
                //     None => bottom += "---+",
                // }
            }
            output = output + &format!("{}\n{}\n", &left, &bottom).to_owned();
        }
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{
        Direction, Position,
        cell::Cell,
        grid::{Grid, StandardGrid, Svg},
    };

    #[test]
    fn it_should_create_grid() {
        let grid = StandardGrid::new(4, 4);
        assert_eq!(
            grid,
            StandardGrid {
                links: HashMap::new(),
                width: 4,
                height: 4,
                map: vec![
                    Cell {
                        position: Position { x: 0, y: 0 },
                        neighbours: HashMap::from([
                            (Direction::East, Position { x: 1, y: 0 }),
                            (Direction::South, Position { x: 0, y: 1 })
                        ]),
                    },
                    Cell {
                        position: Position { x: 1, y: 0 },
                        neighbours: HashMap::from([
                            (Direction::South, Position { x: 1, y: 1 }),
                            (Direction::West, Position { x: 0, y: 0 }),
                            (Direction::East, Position { x: 2, y: 0 })
                        ]),
                    },
                    Cell {
                        position: Position { x: 2, y: 0 },
                        neighbours: HashMap::from([
                            (Direction::East, Position { x: 3, y: 0 }),
                            (Direction::South, Position { x: 2, y: 1 }),
                            (Direction::West, Position { x: 1, y: 0 })
                        ]),
                    },
                    Cell {
                        position: Position { x: 3, y: 0 },
                        neighbours: HashMap::from([
                            (Direction::South, Position { x: 3, y: 1 }),
                            (Direction::West, Position { x: 2, y: 0 })
                        ]),
                    },
                    Cell {
                        position: Position { x: 0, y: 1 },
                        neighbours: HashMap::from([
                            (Direction::East, Position { x: 1, y: 1 }),
                            (Direction::South, Position { x: 0, y: 2 }),
                            (Direction::North, Position { x: 0, y: 0 })
                        ]),
                    },
                    Cell {
                        position: Position { x: 1, y: 1 },
                        neighbours: HashMap::from([
                            (Direction::West, Position { x: 0, y: 1 }),
                            (Direction::East, Position { x: 2, y: 1 }),
                            (Direction::North, Position { x: 1, y: 0 }),
                            (Direction::South, Position { x: 1, y: 2 })
                        ]),
                    },
                    Cell {
                        position: Position { x: 2, y: 1 },
                        neighbours: HashMap::from([
                            (Direction::East, Position { x: 3, y: 1 }),
                            (Direction::West, Position { x: 1, y: 1 }),
                            (Direction::North, Position { x: 2, y: 0 }),
                            (Direction::South, Position { x: 2, y: 2 })
                        ]),
                    },
                    Cell {
                        position: Position { x: 3, y: 1 },
                        neighbours: HashMap::from([
                            (Direction::North, Position { x: 3, y: 0 }),
                            (Direction::South, Position { x: 3, y: 2 }),
                            (Direction::West, Position { x: 2, y: 1 })
                        ]),
                    },
                    Cell {
                        position: Position { x: 0, y: 2 },
                        neighbours: HashMap::from([
                            (Direction::North, Position { x: 0, y: 1 }),
                            (Direction::South, Position { x: 0, y: 3 }),
                            (Direction::East, Position { x: 1, y: 2 })
                        ]),
                    },
                    Cell {
                        position: Position { x: 1, y: 2 },
                        neighbours: HashMap::from([
                            (Direction::North, Position { x: 1, y: 1 }),
                            (Direction::East, Position { x: 2, y: 2 }),
                            (Direction::West, Position { x: 0, y: 2 }),
                            (Direction::South, Position { x: 1, y: 3 })
                        ]),
                    },
                    Cell {
                        position: Position { x: 2, y: 2 },
                        neighbours: HashMap::from([
                            (Direction::West, Position { x: 1, y: 2 }),
                            (Direction::South, Position { x: 2, y: 3 }),
                            (Direction::East, Position { x: 3, y: 2 }),
                            (Direction::North, Position { x: 2, y: 1 })
                        ]),
                    },
                    Cell {
                        position: Position { x: 3, y: 2 },
                        neighbours: HashMap::from([
                            (Direction::West, Position { x: 2, y: 2 }),
                            (Direction::North, Position { x: 3, y: 1 }),
                            (Direction::South, Position { x: 3, y: 3 })
                        ]),
                    },
                    Cell {
                        position: Position { x: 0, y: 3 },
                        neighbours: HashMap::from([
                            (Direction::North, Position { x: 0, y: 2 }),
                            (Direction::East, Position { x: 1, y: 3 })
                        ]),
                    },
                    Cell {
                        position: Position { x: 1, y: 3 },
                        neighbours: HashMap::from([
                            (Direction::North, Position { x: 1, y: 2 }),
                            (Direction::East, Position { x: 2, y: 3 }),
                            (Direction::West, Position { x: 0, y: 3 })
                        ]),
                    },
                    Cell {
                        position: Position { x: 2, y: 3 },
                        neighbours: HashMap::from([
                            (Direction::North, Position { x: 2, y: 2 }),
                            (Direction::West, Position { x: 1, y: 3 }),
                            (Direction::East, Position { x: 3, y: 3 })
                        ]),
                    },
                    Cell {
                        position: Position { x: 3, y: 3 },
                        neighbours: HashMap::from([
                            (Direction::West, Position { x: 2, y: 3 }),
                            (Direction::North, Position { x: 3, y: 2 })
                        ]),
                    }
                ]
            }
        );
    }

    #[test]
    fn it_should_link_cells() {
        let mut container = StandardGrid::new(4, 4);
        let next_cell_pos = container.map[1].position;

        container
            .links
            .entry(container.map[0].position)
            .or_insert(vec![])
            .push(next_cell_pos);

        let next_cell_pos = container.map[11].position;
        container
            .links
            .entry(container.map[7].position)
            .or_insert(vec![])
            .push(next_cell_pos);
        let next_cell_pos = container.map[6].position;
        container
            .links
            .entry(container.map[7].position)
            .or_insert(vec![])
            .push(next_cell_pos);

        let mut links = HashMap::new();
        links.insert(Position { x: 0, y: 0 }, vec![Position { x: 1, y: 0 }]);
        links.insert(
            Position { x: 3, y: 1 },
            vec![Position { x: 3, y: 2 }, Position { x: 2, y: 1 }],
        );

        assert_eq!(links, container.links);
    }

    #[test]
    fn it_should_display_ascii() {
        let mut grid = StandardGrid::new(4, 4);
        assert_eq!(
            format!("{}", grid),
            "+---+---+---+---+
|   |   |   |   |
+---+---+---+---+
|   |   |   |   |
+---+---+---+---+
|   |   |   |   |
+---+---+---+---+
|   |   |   |   |
+---+---+---+---+
"
        );

        // note have to connect earlier cell to later
        let next_cell_pos = grid.map[1].position;

        grid.links
            .entry(grid.map[0].position)
            .or_insert(vec![])
            .push(next_cell_pos);

        let next_cell_pos = grid.map[11].position;
        grid.links
            .entry(grid.map[7].position)
            .or_insert(vec![])
            .push(next_cell_pos);

        assert_eq!(
            format!("{}", grid),
            "+---+---+---+---+
|       |   |   |
+---+---+---+---+
|   |   |   |   |
+---+---+---+   +
|   |   |   |   |
+---+---+---+---+
|   |   |   |   |
+---+---+---+---+
"
        );
    }

    #[test]
    fn it_should_draw_svg() {
        let grid = StandardGrid::new(4, 4);
        let svg = StandardGrid::draw(&grid, &grid.map, grid.width, grid.height);
        assert_eq!(
            "<svg viewBox=\"0 0 64 64\" xmlns=\"http://www.w3.org/2000/svg\"><line x1=\"0\" y1=\"0\" x2=\"16\" y2=\"0\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"0\" y1=\"0\" x2=\"0\" y2=\"16\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"16\" y1=\"0\" x2=\"16\" y2=\"16\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"0\" y1=\"16\" x2=\"16\" y2=\"16\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"16\" y1=\"0\" x2=\"32\" y2=\"0\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"32\" y1=\"0\" x2=\"32\" y2=\"16\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"16\" y1=\"16\" x2=\"32\" y2=\"16\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"32\" y1=\"0\" x2=\"48\" y2=\"0\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"48\" y1=\"0\" x2=\"48\" y2=\"16\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"32\" y1=\"16\" x2=\"48\" y2=\"16\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"48\" y1=\"0\" x2=\"64\" y2=\"0\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"64\" y1=\"0\" x2=\"64\" y2=\"16\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"48\" y1=\"16\" x2=\"64\" y2=\"16\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"0\" y1=\"16\" x2=\"0\" y2=\"32\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"16\" y1=\"16\" x2=\"16\" y2=\"32\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"0\" y1=\"32\" x2=\"16\" y2=\"32\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"32\" y1=\"16\" x2=\"32\" y2=\"32\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"16\" y1=\"32\" x2=\"32\" y2=\"32\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"48\" y1=\"16\" x2=\"48\" y2=\"32\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"32\" y1=\"32\" x2=\"48\" y2=\"32\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"64\" y1=\"16\" x2=\"64\" y2=\"32\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"48\" y1=\"32\" x2=\"64\" y2=\"32\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"0\" y1=\"32\" x2=\"0\" y2=\"48\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"16\" y1=\"32\" x2=\"16\" y2=\"48\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"0\" y1=\"48\" x2=\"16\" y2=\"48\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"32\" y1=\"32\" x2=\"32\" y2=\"48\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"16\" y1=\"48\" x2=\"32\" y2=\"48\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"48\" y1=\"32\" x2=\"48\" y2=\"48\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"32\" y1=\"48\" x2=\"48\" y2=\"48\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"64\" y1=\"32\" x2=\"64\" y2=\"48\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"48\" y1=\"48\" x2=\"64\" y2=\"48\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"0\" y1=\"48\" x2=\"0\" y2=\"64\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"16\" y1=\"48\" x2=\"16\" y2=\"64\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"0\" y1=\"64\" x2=\"16\" y2=\"64\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"32\" y1=\"48\" x2=\"32\" y2=\"64\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"16\" y1=\"64\" x2=\"32\" y2=\"64\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"48\" y1=\"48\" x2=\"48\" y2=\"64\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"32\" y1=\"64\" x2=\"48\" y2=\"64\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"64\" y1=\"48\" x2=\"64\" y2=\"64\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"48\" y1=\"64\" x2=\"64\" y2=\"64\" stroke=\"black\" stroke-linecap=\"square\" /></svg>",
            svg
        );
    }
}
