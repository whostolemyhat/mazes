use std::collections::HashMap;

use crate::{Direction, Position};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Cell {
    pub position: Position,
    pub neighbours: HashMap<Direction, Position>,
    pub links: HashMap<Position, bool>,
}

impl Cell {
    pub fn new(x: i32, y: i32) -> Self {
        Cell {
            position: Position { x, y },
            neighbours: HashMap::new(),
            links: HashMap::new(),
        }
    }

    pub fn link(&mut self, cell_pos: &Position) {
        self.links.insert(*cell_pos, true);
    }

    pub fn unlink(&mut self, cell_pos: &Position) {
        self.links.remove(cell_pos);
    }

    pub fn is_linked(&self, pos: &Position) -> bool {
        self.links.contains_key(pos)
    }

    pub fn get_neighbours(&self) -> Vec<&Position> {
        self.neighbours.values().collect()
    }
}
