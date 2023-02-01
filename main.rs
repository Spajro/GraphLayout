mod force_driven_layout;
mod graph;
mod math2d;
mod input_csv;
mod display;

extern crate core;

use crate::graph::*;
use crate::force_driven_layout::State;
use crate::math2d::Position;
use crate::force_driven_layout::iterate;
use crate::input_csv::input;

fn main() {
    test_with_input()
}

fn test_with_input() {
    let state = &mut State {
        graph: input("examples/square.csv".to_string()).unwrap(),

        positions: Default::default(),
    };
    state.positions.insert(Vertex { id: 1, label: "alpha".to_string() }, Position { x: 100, y: 100 });
    state.positions.insert(Vertex { id: 2, label: "beta".to_string() }, Position { x: 100, y: 300 });
    state.positions.insert(Vertex { id: 3, label: "gamma".to_string() }, Position { x: 300, y: 300 });
    state.positions.insert(Vertex { id: 4, label: "epsilon".to_string() }, Position { x: 300, y: 100 });
    for _i in 0..5 {
        display(state);
        iterate(state);
    }
}

fn test() {
    let state = &mut State {
        graph: Graph {
            vertexes: vec![Vertex { id: 1, label: "alpha".to_string() },
                           Vertex { id: 2, label: "beta".to_string() },
                           Vertex { id: 3, label: "gamma".to_string() },
                           Vertex { id: 4, label: "epsilon".to_string() }],
            edges: vec![
                Edge {
                    first: Vertex { id: 1, label: "alpha".to_string() },
                    second: Vertex { id: 2, label: "beta".to_string() },
                },
                Edge {
                    first: Vertex { id: 2, label: "beta".to_string() },
                    second: Vertex { id: 3, label: "gamma".to_string() },
                },
                Edge {
                    first: Vertex { id: 3, label: "gamma".to_string() },
                    second: Vertex { id: 4, label: "epsilon".to_string() },
                },
                Edge {
                    first: Vertex { id: 4, label: "epsilon".to_string() },
                    second: Vertex { id: 1, label: "alpha".to_string() },
                }],
        },
        positions: Default::default(),
    };
    state.positions.insert(Vertex { id: 1, label: "alpha".to_string() }, Position { x: 100, y: 100 });
    state.positions.insert(Vertex { id: 2, label: "beta".to_string() }, Position { x: 100, y: 300 });
    state.positions.insert(Vertex { id: 3, label: "gamma".to_string() }, Position { x: 300, y: 300 });
    state.positions.insert(Vertex { id: 4, label: "epsilon".to_string() }, Position { x: 300, y: 100 });
    println!("WORK");
    for _i in 0..5 {
        display(state);
        iterate(state);
    }
}

fn display(state: &State) {
    println!();
    println!("1: {},{}", state.positions.get(&Vertex { id: 1, label: "alpha".to_string() }).unwrap().x, state.positions.get(&Vertex { id: 1, label: "alpha".to_string() }).unwrap().y);
    println!("2: {},{}", state.positions.get(&Vertex { id: 2, label: "beta".to_string() }).unwrap().x, state.positions.get(&Vertex { id: 2, label: "beta".to_string() }).unwrap().y);
    println!("3: {},{}", state.positions.get(&Vertex { id: 3, label: "gamma".to_string() }).unwrap().x, state.positions.get(&Vertex { id: 3, label: "gamma".to_string() }).unwrap().y);
    println!("4: {},{}", state.positions.get(&Vertex { id: 4, label: "epsilon".to_string() }).unwrap().x, state.positions.get(&Vertex { id: 4, label: "epsilon".to_string() }).unwrap().y);
}



