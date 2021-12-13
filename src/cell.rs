use std::collections::HashMap;

pub type Position = (i32, i32);

#[derive(Clone, Default, Debug)]
pub struct Neighbours {
  pub north: Option<Position>,
  pub south: Option<Position>,
  pub east: Option<Position>,
  pub west: Option<Position>,
}

#[derive(Clone, Debug)]
pub struct Cell {
  pub row: i32,
  pub column: i32,
  pub links: HashMap<Position, bool>,
  pub neighbours: Neighbours,
}

impl Cell {
  pub fn new(row: i32, column: i32) -> Self {
    Cell {
      row,
      column,
      links: HashMap::new(),
      neighbours: Default::default(),
    }
  }

  fn position(&self) -> Position {
    (self.row, self.column)
  }

  // removed bi-directional stuff
  pub fn link(&mut self, cell_pos: &Position) {
    self.links.insert(*cell_pos, true);
  }

  fn unlink(&mut self, cell_pos: &Position) {
    self.links.remove(&cell_pos);
  }

  // fn links() {}

  fn is_linked(&self, pos: Position) -> bool {
    self.links.contains_key(&pos)
  }
}
