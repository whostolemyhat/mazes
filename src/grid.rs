use std::fmt::{self, Display, Formatter};

use rand::Rng;

use crate::{Direction, Position, cell::Cell};

#[derive(Debug, Eq, PartialEq)]
pub struct Grid {
    pub map: Vec<Cell>,
    pub width: i32,
    pub height: i32,
}

impl Grid {
    pub fn new(width: i32, height: i32) -> Self {
        let mut map = Grid::prepare_map(width, height);
        Grid::configure_cells(&mut map, width, height);
        Grid { map, width, height }
    }

    pub fn random_cell(&self) -> Cell {
        let mut rng = rand::rng();
        let y = rng.random_range(0..self.height);
        let x = rng.random_range(0..self.width);

        self.map[((y * self.width) + x) as usize].clone()
    }

    pub fn size(&self) -> i32 {
        self.width * self.height
    }

    pub fn draw(&self) -> String {
        let wall_colour = "black";
        let cell_size = 16;
        let mut output = String::new();

        self.map.iter().for_each(|cell| {
            let x1 = cell.position.x * cell_size;
            let y1 = cell.position.y * cell_size;
            let x2 = (cell.position.x + 1) * cell_size;
            let y2 = (cell.position.y + 1) * cell_size;

            // draw north and west if there are no cells in that direction
            // (ie outside)
            let north = cell.neighbours.get(&Direction::North);
            if north.is_none() {
                output += &Self::svg_line(x1, y1, x2, y1, wall_colour);
            }

            let west = cell.neighbours.get(&Direction::West);
            if west.is_none() {
                output += &Self::svg_line(x1, y1, x1, y2, wall_colour);
            }

            // draw east and south if there is no cell (outside)
            // or if there is no link to that direction
            let east = cell.neighbours.get(&Direction::East);
            if east.is_none() || (east.is_some() && !cell.is_linked(east.unwrap())) {
                output += &Self::svg_line(x2, y1, x2, y2, wall_colour);
            }

            let south = cell.neighbours.get(&Direction::South);
            if south.is_none() || (south.is_some() && !cell.is_linked(south.unwrap())) {
                output += &Self::svg_line(x1, y2, x2, y2, wall_colour);
            }
        });
        let svg = format!(
            "<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">{}</svg>",
            cell_size * self.width,
            cell_size * self.height,
            output
        );
        svg
    }

    fn svg_line(x1: i32, y1: i32, x2: i32, y2: i32, wall_colour: &str) -> String {
        format!(
            "<line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\" stroke=\"{wall_colour}\" stroke-linecap=\"square\" />"
        )
    }
}

// separate so can be overridden for different types of grid
trait GridSetup {
    fn prepare_map(width: i32, height: i32) -> Vec<Cell>;
    fn configure_cells(map: &mut Vec<Cell>, width: i32, height: i32);
}

impl GridSetup for Grid {
    fn prepare_map(width: i32, height: i32) -> Vec<Cell> {
        let mut map = vec![];
        for y in 0..height {
            for x in 0..width {
                map.push(Cell::new(x, y))
            }
        }
        map
    }

    fn configure_cells(map: &mut Vec<Cell>, width: i32, height: i32) {
        for cell in map.iter_mut() {
            let row = cell.position.y;
            let col = cell.position.x;

            if row > 0 {
                cell.neighbours
                    .insert(Direction::North, Position { x: row - 1, y: col });
            }

            if row < height - 1 {
                cell.neighbours
                    .insert(Direction::South, Position { x: row + 1, y: col });
            }

            if col > 0 {
                cell.neighbours
                    .insert(Direction::West, Position { x: row, y: col - 1 });
            }
            if col < width - 1 {
                cell.neighbours
                    .insert(Direction::East, Position { x: row, y: col + 1 });
            }
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut output: String = "+".to_owned();
        for _ in 0..self.width {
            output += "---+"
        }
        output += "\n";

        for row in 0..self.height {
            let mut top = "|".to_owned();
            let mut bottom = "+".to_owned();

            for col in 0..self.width {
                let index = ((row * self.width) + col) as usize;
                let cell = self.map[index].clone();
                top += "   ";
                let east = cell.links.get(&Position { x: row, y: col + 1 });
                dbg!(&east);
                match east {
                    Some(_) => top += " ",
                    None => top += "|",
                }
                let south = cell.links.get(&Position { x: row + 1, y: col });
                match south {
                    Some(_) => bottom += "   +",
                    None => bottom += "---+",
                }
            }
            output = output + &format!("{}\n{}\n", &top, &bottom).to_owned();
        }
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{Direction, Grid, Position, cell::Cell};

    #[test]
    fn it_should_create_grid() {
        let grid = Grid::new(4, 4);
        assert_eq!(
            grid,
            Grid {
                width: 4,
                height: 4,
                map: vec![
                    Cell {
                        position: Position { x: 0, y: 0 },
                        neighbours: HashMap::from([
                            (Direction::East, Position { x: 0, y: 1 }),
                            (Direction::South, Position { x: 1, y: 0 })
                        ]),
                        links: HashMap::new()
                    },
                    Cell {
                        position: Position { x: 1, y: 0 },
                        neighbours: HashMap::from([
                            (Direction::South, Position { x: 1, y: 1 }),
                            (Direction::West, Position { x: 0, y: 0 }),
                            (Direction::East, Position { x: 0, y: 2 })
                        ]),
                        links: HashMap::new()
                    },
                    Cell {
                        position: Position { x: 2, y: 0 },
                        neighbours: HashMap::from([
                            (Direction::West, Position { x: 0, y: 1 }),
                            (Direction::East, Position { x: 0, y: 3 }),
                            (Direction::South, Position { x: 1, y: 2 })
                        ]),
                        links: HashMap::new()
                    },
                    Cell {
                        position: Position { x: 3, y: 0 },
                        neighbours: HashMap::from([
                            (Direction::South, Position { x: 1, y: 3 }),
                            (Direction::West, Position { x: 0, y: 2 })
                        ]),
                        links: HashMap::new()
                    },
                    Cell {
                        position: Position { x: 0, y: 1 },
                        neighbours: HashMap::from([
                            (Direction::South, Position { x: 2, y: 0 }),
                            (Direction::East, Position { x: 1, y: 1 }),
                            (Direction::North, Position { x: 0, y: 0 })
                        ]),
                        links: HashMap::new()
                    },
                    Cell {
                        position: Position { x: 1, y: 1 },
                        neighbours: HashMap::from([
                            (Direction::South, Position { x: 2, y: 1 }),
                            (Direction::East, Position { x: 1, y: 2 }),
                            (Direction::North, Position { x: 0, y: 1 }),
                            (Direction::West, Position { x: 1, y: 0 })
                        ]),
                        links: HashMap::new()
                    },
                    Cell {
                        position: Position { x: 2, y: 1 },
                        neighbours: HashMap::from([
                            (Direction::South, Position { x: 2, y: 2 }),
                            (Direction::North, Position { x: 0, y: 2 }),
                            (Direction::West, Position { x: 1, y: 1 }),
                            (Direction::East, Position { x: 1, y: 3 })
                        ]),
                        links: HashMap::new()
                    },
                    Cell {
                        position: Position { x: 3, y: 1 },
                        neighbours: HashMap::from([
                            (Direction::North, Position { x: 0, y: 3 }),
                            (Direction::South, Position { x: 2, y: 3 }),
                            (Direction::West, Position { x: 1, y: 2 })
                        ]),
                        links: HashMap::new()
                    },
                    Cell {
                        position: Position { x: 0, y: 2 },
                        neighbours: HashMap::from([
                            (Direction::South, Position { x: 3, y: 0 }),
                            (Direction::East, Position { x: 2, y: 1 }),
                            (Direction::North, Position { x: 1, y: 0 })
                        ]),
                        links: HashMap::new()
                    },
                    Cell {
                        position: Position { x: 1, y: 2 },
                        neighbours: HashMap::from([
                            (Direction::East, Position { x: 2, y: 2 }),
                            (Direction::West, Position { x: 2, y: 0 }),
                            (Direction::North, Position { x: 1, y: 1 }),
                            (Direction::South, Position { x: 3, y: 1 })
                        ]),
                        links: HashMap::new()
                    },
                    Cell {
                        position: Position { x: 2, y: 2 },
                        neighbours: HashMap::from([
                            (Direction::East, Position { x: 2, y: 3 }),
                            (Direction::North, Position { x: 1, y: 2 }),
                            (Direction::South, Position { x: 3, y: 2 }),
                            (Direction::West, Position { x: 2, y: 1 }),
                        ]),
                        links: HashMap::new()
                    },
                    Cell {
                        position: Position { x: 3, y: 2 },
                        neighbours: HashMap::from([
                            (Direction::West, Position { x: 2, y: 2 }),
                            (Direction::North, Position { x: 1, y: 3 }),
                            (Direction::South, Position { x: 3, y: 3 }),
                        ]),
                        links: HashMap::new()
                    },
                    Cell {
                        position: Position { x: 0, y: 3 },
                        neighbours: HashMap::from([
                            (Direction::North, Position { x: 2, y: 0 }),
                            (Direction::East, Position { x: 3, y: 1 }),
                        ]),
                        links: HashMap::new()
                    },
                    Cell {
                        position: Position { x: 1, y: 3 },
                        neighbours: HashMap::from([
                            (Direction::North, Position { x: 2, y: 1 }),
                            (Direction::West, Position { x: 3, y: 0 }),
                            (Direction::East, Position { x: 3, y: 2 }),
                        ]),
                        links: HashMap::new()
                    },
                    Cell {
                        position: Position { x: 2, y: 3 },
                        neighbours: HashMap::from([
                            (Direction::West, Position { x: 3, y: 1 }),
                            (Direction::East, Position { x: 3, y: 3 }),
                            (Direction::North, Position { x: 2, y: 2 }),
                        ]),
                        links: HashMap::new()
                    },
                    Cell {
                        position: Position { x: 3, y: 3 },
                        neighbours: HashMap::from([
                            (Direction::West, Position { x: 3, y: 2 }),
                            (Direction::North, Position { x: 2, y: 3 })
                        ]),
                        links: HashMap::new()
                    }
                ]
            }
        );
    }

    #[test]
    fn it_should_display_ascii() {
        let mut grid = Grid::new(2, 1);
        //         assert_eq!(
        //             format!("{}", grid),
        //             "+---+---+---+---+
        // |   |   |   |   |
        // +---+---+---+---+
        // |   |   |   |   |
        // +---+---+---+---+
        // "
        //         );

        let next_cell_pos = grid.map[1].position;
        grid.map[0].link(&next_cell_pos);

        dbg!(&grid);

        assert_eq!(
            format!("{}", grid),
            "+---+---+---+---+
|   |       |   |
+---+---+---+---+
|   |   |   |   |
+---+---+---+---+
"
        );
    }

    #[test]
    fn it_should_draw_svg() {
        let grid = Grid::new(4, 4);
        let svg = grid.draw();
        assert_eq!(
            "<svg viewBox=\"0 0 64 64\" xmlns=\"http://www.w3.org/2000/svg\"><line x1=\"0\" y1=\"0\" x2=\"16\" y2=\"0\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"0\" y1=\"0\" x2=\"0\" y2=\"16\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"16\" y1=\"0\" x2=\"16\" y2=\"16\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"0\" y1=\"16\" x2=\"16\" y2=\"16\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"16\" y1=\"0\" x2=\"32\" y2=\"0\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"32\" y1=\"0\" x2=\"32\" y2=\"16\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"16\" y1=\"16\" x2=\"32\" y2=\"16\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"32\" y1=\"0\" x2=\"48\" y2=\"0\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"48\" y1=\"0\" x2=\"48\" y2=\"16\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"32\" y1=\"16\" x2=\"48\" y2=\"16\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"48\" y1=\"0\" x2=\"64\" y2=\"0\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"64\" y1=\"0\" x2=\"64\" y2=\"16\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"48\" y1=\"16\" x2=\"64\" y2=\"16\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"0\" y1=\"16\" x2=\"0\" y2=\"32\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"16\" y1=\"16\" x2=\"16\" y2=\"32\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"0\" y1=\"32\" x2=\"16\" y2=\"32\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"32\" y1=\"16\" x2=\"32\" y2=\"32\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"16\" y1=\"32\" x2=\"32\" y2=\"32\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"48\" y1=\"16\" x2=\"48\" y2=\"32\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"32\" y1=\"32\" x2=\"48\" y2=\"32\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"64\" y1=\"16\" x2=\"64\" y2=\"32\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"48\" y1=\"32\" x2=\"64\" y2=\"32\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"0\" y1=\"32\" x2=\"0\" y2=\"48\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"16\" y1=\"32\" x2=\"16\" y2=\"48\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"0\" y1=\"48\" x2=\"16\" y2=\"48\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"32\" y1=\"32\" x2=\"32\" y2=\"48\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"16\" y1=\"48\" x2=\"32\" y2=\"48\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"48\" y1=\"32\" x2=\"48\" y2=\"48\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"32\" y1=\"48\" x2=\"48\" y2=\"48\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"64\" y1=\"32\" x2=\"64\" y2=\"48\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"48\" y1=\"48\" x2=\"64\" y2=\"48\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"0\" y1=\"48\" x2=\"0\" y2=\"64\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"16\" y1=\"48\" x2=\"16\" y2=\"64\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"0\" y1=\"64\" x2=\"16\" y2=\"64\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"32\" y1=\"48\" x2=\"32\" y2=\"64\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"16\" y1=\"64\" x2=\"32\" y2=\"64\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"48\" y1=\"48\" x2=\"48\" y2=\"64\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"32\" y1=\"64\" x2=\"48\" y2=\"64\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"64\" y1=\"48\" x2=\"64\" y2=\"64\" stroke=\"black\" stroke-linecap=\"square\" /><line x1=\"48\" y1=\"64\" x2=\"64\" y2=\"64\" stroke=\"black\" stroke-linecap=\"square\" /></svg>",
            svg
        );
    }
}
