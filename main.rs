mod force_driven_layout;
mod graph;
mod vectors;
mod input_csv;
mod display;
mod config;

extern crate core;

use std::env;
use rand::random;
use crate::config::Config;
use crate::display::draw;
use crate::force_driven_layout::State;
use crate::vectors::Position;
use crate::force_driven_layout::iterate;
use crate::input_csv::input;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let path = args.get(1).unwrap().clone();
        let state = &mut prepare_state(path);

        for i in 0..state.config.iterations {
            println!("[{}/{}] iterating",
                     i,
                     state.config.iterations
            );
            iterate(state);
        }
        draw(state);
    } else {
        println!("Usage: ./GraphLayout 'file.csv'");
    }
}

fn prepare_state(graph_file_path: String) -> State {
    let mut state = State {
        graph: input(graph_file_path).unwrap(),
        positions: Default::default(),
        config: Default::default(),
    };

    state.graph.vertexes.iter()
        .for_each(|vertex| { state.positions.insert(vertex.clone(), random_position(&state.config)); });

    state
}

fn random_position(config: &Config) -> Position {
    Position {
        x: (100 + (random::<u32>() % config.draw_width())) as i32,
        y: (100 + (random::<u32>() % config.draw_height())) as i32,
    }
}


