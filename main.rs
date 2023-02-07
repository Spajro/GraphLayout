mod force_driven_layout;
mod graph;
mod vectors;
mod input_csv;
mod display;

extern crate core;

use std::env;
use rand::random;
use crate::display::draw;
use crate::force_driven_layout::State;
use crate::vectors::Position;
use crate::force_driven_layout::iterate;
use crate::input_csv::input;

static ITERATIONS: i32 = 100;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let path = args.get(1).unwrap().clone();
        let state = &mut prepare_state(path);

        for i in 0..ITERATIONS {
            println!("[{}/{}] iterating",
                     i,
                     ITERATIONS
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
    };

    state.graph.vertexes.iter()
        .for_each(|vertex| { state.positions.insert(vertex.clone(), random_position()); });

    state
}

fn random_position() -> Position {
    Position {
        x: 100 + (random::<i32>() % 1000),
        y: 100 + (random::<i32>() % 1000),
    }
}


