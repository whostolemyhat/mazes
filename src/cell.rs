use std::collections::{HashMap, HashSet};

use crate::{Direction, Position};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Cell {
    pub position: Position,
    pub neighbours: HashMap<Direction, Position>,
    // pub links: HashSet<Position>,
}

impl Cell {
    pub fn new(x: i32, y: i32) -> Self {
        Cell {
            position: Position { x, y },
            neighbours: HashMap::new(),
            // links: HashSet::new(),
        }
    }

    // TODO link bi-directional
    // pub fn link(&mut self, cell_pos: &Position) {
    //     self.links.insert(*cell_pos);
    // }

    // pub fn unlink(&mut self, cell_pos: &Position) {
    //     self.links.remove(cell_pos);
    // }

    // pub fn is_linked(&self, pos: &Position) -> bool {
    //     self.links.contains(pos)
    // }

    pub fn get_neighbours(&self) -> Vec<&Position> {
        self.neighbours.values().collect()
    }
}
