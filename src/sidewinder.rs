use crate::grid::Grid;
use rand::prelude::*;

pub struct Sidewinder {}

impl Sidewinder {
  pub fn on(grid: &mut Grid) {
    for cell in grid.map.iter_mut() {
      let mut run = vec![];
      // stop at end of row
      let eastern_neighbour = cell.clone().neighbours.east;
      let southern_neighbour = cell.clone().neighbours.south;
      let at_eastern_boundary = eastern_neighbour.is_none();
      let at_southern_boundary = southern_neighbour.is_none();
      let should_close_run = at_eastern_boundary || (!at_southern_boundary && rand::random());
      run.push(cell);

      if should_close_run {
        let mut rng = rand::thread_rng();
        let selected_index = rng.gen_range(0..run.len());
        let selected = &run[selected_index];
        if selected.neighbours.south.is_some() {
          let neighbour_pos = selected
            .neighbours
            .south
            .expect("Couldn't get neighbour position");
          // using 'selected' here causes mut/ref issues
          run[selected_index].link(&neighbour_pos);
        }
        run.clear();
      } else {
        let neighbour_pos = eastern_neighbour.expect("Can't get neighbour pos");
        let last_index = run.len() - 1;
        // as above, operating on cell directly is a no-no
        run[last_index].link(&neighbour_pos);
      }
    }
  }
}
