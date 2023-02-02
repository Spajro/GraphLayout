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
    let state = &mut State {
        graph: input("examples/hexagon.csv".to_string()).unwrap(),
        positions: Default::default(),
    };
    state.graph.vertexes.iter()
        .for_each(|vertex| { state.positions.insert(vertex.clone(), random_position()); });
    for _i in 0..10 {
        iterate(state);
    }
    draw(state);
}

fn random_position() -> Position {
    Position {
        x: 100 + (random::<i32>() % 1000),
        y: 100 + (random::<i32>() % 1000),
    }
}


