use std::collections::HashMap;

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
    distances: HashMap<Position, i32>,
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
            distances: HashMap::new(),
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
        let distances = distances(&self.map[0].position, &self.links());
        self.distances = distances;
        self.links = links;
    }

    fn distances(&self) -> HashMap<Position, i32> {
        self.distances.clone()
    }

    fn set_distances(&mut self, distances: HashMap<Position, i32>) {
        self.distances = distances;
    }

    fn contents_of(&self, cell: &Cell) -> String {
        dbg!(&cell.position, self.distances.get(&cell.position));

        match self.distances.get(&cell.position) {
            Some(num) => match std::char::from_digit(*num as u32, 36) {
                Some(ch) => ch.to_string(),
                None => String::from(" "),
            },
            _ => String::from(" "),
        }
    }
}

impl GridSetup for DjikstraGrid {}

impl Svg for DjikstraGrid {}
