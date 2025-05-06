use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

use crate::{
    Position,
    base_grid::{GridSetup, Svg},
    cell::Cell,
    distances::distances,
    grid::Grid,
};

#[derive(Debug, Eq, PartialEq)]
pub struct DjikstraGrid {
    pub map: Vec<Cell>,
    pub width: i32,
    pub height: i32,
    pub links: HashMap<Position, Vec<Position>>,
}

impl DjikstraGrid {
    pub fn new(width: i32, height: i32) -> Self {
        let mut map = Self::prepare_map(width, height);
        Self::configure_cells(&mut map, width, height);
        let links = HashMap::new();
        Self {
            map,
            width,
            height,
            links,
        }
    }
}

impl Grid for DjikstraGrid {
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

impl GridSetup for DjikstraGrid {
    fn contents_of(&self, cell: &Cell) -> String {
        let distances = distances(&self.map[0].position, self);

        match distances.get(&cell.position) {
            Some(num) => match std::char::from_digit(*num as u32, 36) {
                Some(ch) => ch.to_string(),
                None => String::from(" "),
            },
            _ => String::from(" "),
        }
    }
}

impl Svg for DjikstraGrid {}

// TODO orpha rule/macro
impl Display for DjikstraGrid {
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

// impl Display for DjikstraGrid {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         let mut output: String = "+".to_owned();
//         for _ in 0..self.width {
//             output += "---+"
//         }
//         output += "\n";

//         for row in 0..self.height {
//             let mut top = "|".to_owned();
//             let mut bottom = "+".to_owned();

//             for col in 0..self.width {
//                 let index = ((row * self.width) + col) as usize;
//                 let cell = self.map[index].clone();
//                 let body = self.contents_of(&cell);
//                 top += &format!(" {} ", body);
//                 let east = cell.links.get(&Position { x: col + 1, y: row });
//                 match east {
//                     Some(_) => top += " ",
//                     None => top += "|",
//                 }
//                 let south = cell.links.get(&Position { x: col, y: row + 1 });
//                 match south {
//                     Some(_) => bottom += "   +",
//                     None => bottom += "---+",
//                 }
//             }
//             output = output + &format!("{}\n{}\n", &top, &bottom).to_owned();
//         }
//         write!(f, "{}", output)
//     }
// }
