use crate::{Direction, Position, cell::Cell, grid::Grid};

// TODO grid owns links
// TODO sub-trait?
// pub trait BaseGrid {
//     fn width(&self) -> i32;
//     fn height(&self) -> i32;
//     fn map(&self) -> &Vec<Cell>;
//     fn map_mut(&mut self) -> &mut Vec<Cell>;
//     fn links(&mut self) -> &mut RefCell<HashMap<Position, Vec<Position>>>;

//     fn random_cell(&self) -> Cell {
//         let mut rng = rand::rng();
//         let y = rng.random_range(0..self.height());
//         let x = rng.random_range(0..self.width());

//         self.map()[((y * self.width()) + x) as usize].clone()
//     }

//     fn cell_at(&mut self, pos: &Position) -> Option<&mut Cell> {
//         let map = self.map_mut();
//         map.iter_mut().find(|cell| cell.position == *pos)
//     }

//     fn size(&self) -> i32 {
//         self.width() * self.height()
//     }

//     fn contents_of(&self, _cell: &Cell) -> String {
//         String::from(" ")
//     }
// }

pub trait Svg {
    fn draw(grid: &Grid, map: &Vec<Cell>, width: i32, height: i32) -> String {
        let wall_colour = "black";
        let cell_size = 16;
        let mut output = String::new();

        map.iter().for_each(|cell| {
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
            if east.is_none() || (east.is_some() && !grid.is_linked(&cell.position, east.unwrap()))
            {
                output += &Self::svg_line(x2, y1, x2, y2, wall_colour);
            }

            let south = cell.neighbours.get(&Direction::South);
            if south.is_none()
                || (south.is_some() && !grid.is_linked(&cell.position, south.unwrap()))
            {
                output += &Self::svg_line(x1, y2, x2, y2, wall_colour);
            }
        });
        let svg = format!(
            "<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">{}</svg>",
            cell_size * width,
            cell_size * height,
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
pub trait GridSetup {
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
                    .insert(Direction::North, Position { x: col, y: row - 1 });
            }

            if row < height - 1 {
                cell.neighbours
                    .insert(Direction::South, Position { x: col, y: row + 1 });
            }

            if col > 0 {
                cell.neighbours
                    .insert(Direction::West, Position { x: col - 1, y: row });
            }
            if col < width - 1 {
                cell.neighbours
                    .insert(Direction::East, Position { x: col + 1, y: row });
            }
        }
    }

    fn contents_of(&self, _cell: &Cell) -> String {
        String::from(" ")
    }
}
