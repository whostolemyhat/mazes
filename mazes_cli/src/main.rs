use clap::Parser;
use serde::Serialize;
use std::{
    fs::write,
    io,
    time::{SystemTime, UNIX_EPOCH},
};

use mazes::{
    Position,
    algos::{binary_tree::binary_tree, sidewinder::sidewinder},
    base_grid::Svg,
    distances::path_to,
    djikstra::DjikstraGrid,
    grid::{Grid, StandardGrid},
};
use rand::rngs::SmallRng;
use rand_seeder::Seeder;

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Algos {
    Sidewinder,
    #[default]
    Binary,
}

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum GridType {
    #[default]
    Standard,
    Djikstra,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct GenerateArgs {
    #[arg(short, long, default_value_t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Somehow time has failed")
        .as_millis().to_string())]
    seed: String,
    #[arg(short, long, default_value_t, value_enum)]
    grid: GridType,
    #[arg(short, long, default_value_t, value_enum)]
    algo: Algos,
    #[arg(short = 'x', long, default_value_t = 8)]
    width: usize,
    #[arg(short = 'y', long, default_value_t = 8)]
    height: usize,
}

fn main() -> Result<(), io::Error> {
    let gen_args = GenerateArgs::parse();
    let mut rng: SmallRng = Seeder::from(&gen_args.seed).into_rng();

    let mut grid: Box<dyn Grid> = match gen_args.grid {
        GridType::Standard => Box::new(StandardGrid::new(
            gen_args.width as i32,
            gen_args.height as i32,
        )),
        GridType::Djikstra => Box::new(DjikstraGrid::new(
            gen_args.width as i32,
            gen_args.height as i32,
        )),
    };

    match gen_args.algo {
        Algos::Sidewinder => sidewinder(&mut grid, &mut rng),
        Algos::Binary => binary_tree(&mut grid, &mut rng),
    };

    if gen_args.grid == GridType::Djikstra {
        // find a path and update grid distances
        // only works for djikstra
        let path = path_to(&Position { x: 5, y: 1 }, &grid);
        grid.set_distances(path);
    }
    println!("{}", grid);

    let output = grid.draw(&grid.map(), grid.width(), grid.height());
    write("./test.svg", output)?;

    Ok(())
}
