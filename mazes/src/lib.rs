// use std::{fs::write, io};

// use algos::{binary_tree::binary_tree, sidewinder::sidewinder};
// use base_grid::Svg;
// use djikstra::DjikstraGrid;
// use grid::StandardGrid;
// use rand::rngs::SmallRng;
// use rand_seeder::Seeder;

pub mod algos;
pub mod base_grid;
mod cell;
pub mod distances;
pub mod djikstra;
pub mod grid;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
