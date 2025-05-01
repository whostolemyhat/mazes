use std::collections::HashMap;

use crate::{
    base_grid::{GridSetup, Svg},
    cell::Cell,
    distances::distances,
    grid::Grid,
};

#[derive(Debug, Eq, PartialEq)]
pub struct DjikstraGrid {
    pub grid: Grid,
}

impl DjikstraGrid {
    pub fn new(width: i32, height: i32) -> Self {
        let mut map = Grid::prepare_map(width, height);
        Grid::configure_cells(&mut map, width, height);

        DjikstraGrid {
            grid: Grid {
                map,
                width,
                height,
                links: HashMap::new(),
            },
        }
    }
}

impl GridSetup for DjikstraGrid {
    fn contents_of(&self, cell: &Cell) -> String {
        dbg!(&self.grid.map[0]);
        let distances = distances(&self.grid.map[0].position, &self.grid);
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
