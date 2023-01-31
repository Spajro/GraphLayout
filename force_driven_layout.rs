use std::collections::HashMap;
use crate::graph::*;

#[derive(Copy, Clone)]
pub struct Position {
    pub(crate) x: u32,
    pub(crate) y: u32,
}

struct Vector {
    first: Position,
    second: Position,
}

#[derive(Clone)]
pub struct State {
    pub(crate) graph: Graph,
    pub(crate) positions: HashMap<Vertex, Position>,
}

#[derive(Copy, Clone)]
struct Force {
    vertex: Vertex,
    new_position: Position,
}

#[derive(Copy, Clone)]
struct EdgeForce {
    first_force: Force,
    second_force: Force,
}

pub fn iterate(state: &mut State) {
    for edge in state.graph.edges.clone() {
        apply_edge_force_to_state(edge_to_edge_force(edge, state), state);
    }
}

fn edge_to_edge_force(edge: Edge, state: &State) -> EdgeForce {
    let first_position = *state.positions.get(&edge.first).unwrap();
    let second_position = *state.positions.get(&edge.second).unwrap();
    return EdgeForce {
        first_force: Force {
            vertex: edge.first,
            new_position: scale(Vector { first: first_position, second: second_position }).second,
        },
        second_force: Force {
            vertex: edge.second,
            new_position: scale(Vector { first: second_position, second: first_position }).second,
        },
    };
}

fn scale(vector: Vector) -> Vector {
    let cnst: f64 = 0.4;
    Vector {
        first: vector.first,
        second: Position {
            x: (vector.second.x.abs_diff(vector.first.x) as f64 * cnst) as u32,
            y: (vector.second.y.abs_diff(vector.first.y) as f64 * cnst) as u32,
        },
    }
}

fn apply_edge_force_to_state(edge_force: EdgeForce, state: &mut State) {
    apply_force_to_state(edge_force.first_force, state);
    apply_force_to_state(edge_force.second_force, state);
}

fn apply_force_to_state(force: Force, state: &mut State) {
    state.positions.insert(force.vertex, force.new_position);
}
