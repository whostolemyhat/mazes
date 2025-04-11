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
            output = output + "---+"
        }
        output = output + "\n";

        for row in 0..self.height {
            let mut top = "|".to_owned();
            let mut bottom = "+".to_owned();

            for col in 0..self.width {
                let index = ((row * self.width) + col) as usize;
                let cell = self.map[index].clone();
                top = top + "   ";
                let east = cell.links.get(&Position { x: row, y: col + 1 });
                match east {
                    Some(_) => top = top + " ",
                    None => top += "|",
                }
                let south = cell.links.get(&Position { x: row + 1, y: col });
                match south {
                    Some(_) => bottom = bottom + "   +",
                    None => bottom = bottom + "---+",
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
        let mut grid = Grid::new(4, 4);
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

        let next_cell_pos = grid.map[2].position;
        // TODO use pos + index?
        // let mut next_cell = grid.map[2];
        // grid.map[2].link(&mut next_cell, true);
        grid.map[1].links.insert(next_cell_pos, true);

        assert_eq!(
            format!("{}", grid),
            "+---+---+---+---+
|   |       |   |
+---+---+---+---+
|   |   |   |   |
+---+---+---+---+
|   |   |   |   |
+---+---+---+---+
|   |   |   |   |
+---+---+---+---+
"
        );
    }
}
