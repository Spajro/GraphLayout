mod force_driven_layout;
mod graph;
mod math2d;
mod input_csv;
mod display;

extern crate core;

use rand::random;
use crate::display::draw;
use crate::force_driven_layout::State;
use crate::math2d::Position;
use crate::force_driven_layout::iterate;
use crate::input_csv::input;

fn main() {
    let state = &mut prepare_state("examples/hexagon.csv".to_string());
    for i in 0..20 {
        println!("[{}/{}] iterating",
                 i,
                 20
        );
        iterate(state);
        display::adjust(state);
    }
    draw(state);
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


