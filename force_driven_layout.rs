use std::collections::{HashMap, LinkedList};
use crate::graph::*;
use crate::math2d::*;

#[derive(Clone)]
pub struct State {
    pub(crate) graph: Graph,
    pub(crate) positions: HashMap<Vertex, Position>,
}

#[derive(Clone)]
struct Force {
    vertex: Vertex,
    position_diff: NormalizedVector,
}

#[derive(Clone)]
struct ForcePair {
    first_force: Force,
    second_force: Force,
}

static EDGE_FORCE: f64 = 0.3;

pub fn iterate(state: &mut State) {
    let mut force_pairs = LinkedList::new();
    let mut forces = LinkedList::new();
    let mut joined: HashMap<Vertex, NormalizedVector> = HashMap::new();

    state.graph.edges.iter()
        .for_each(|edge| force_pairs.push_back(edge_to_force_pair(edge, state)));

    state.graph.vertexes.iter()
        .for_each(|v1| state.graph.vertexes.iter()
            .for_each(|v2| force_pairs.push_back(vertexes_pair_to_force_pair(v1, v2, state))));

    force_pairs.iter()
        .for_each(|force_pair| {
            forces.push_back(force_pair.first_force.clone());
            forces.push_back(force_pair.second_force.clone());
        });

    state.graph.vertexes.iter()
        .for_each(|vertex| { joined.insert(vertex.clone(), NormalizedVector { x: 0, y: 0 }); });

    forces.iter()
        .for_each(|force| {
            joined.insert(force.vertex.clone(),
                          join(*joined.get(&force.vertex).unwrap(), force.position_diff));
        });

    joined.iter().map(|pair| Force { vertex: pair.0.clone(), position_diff: *pair.1 })
        .for_each(|force| apply_force_to_state(force, state));
}

fn edge_to_force_pair(edge: &Edge, state: &State) -> ForcePair {
    let first_position = *state.positions.get(&edge.first).unwrap();
    let second_position = *state.positions.get(&edge.second).unwrap();
    let cnst: f64 = EDGE_FORCE;
    return ForcePair {
        first_force: Force {
            vertex: edge.first.clone(),
            position_diff: scale_normalized_vector(to_normalized(Vector { first: first_position, second: second_position }), cnst),
        },
        second_force: Force {
            vertex: edge.second.clone(),
            position_diff: scale_normalized_vector(to_normalized(Vector { first: second_position, second: first_position }), cnst),
        },
    };
}

fn vertexes_pair_to_force_pair(first: &Vertex, second: &Vertex, state: &State) -> ForcePair {
    let first_position = *state.positions.get(first).unwrap();
    let second_position = *state.positions.get(second).unwrap();
    let cnst: f64 = EDGE_FORCE * (edge_forces_count(state) as f64) / (vertex_forces_count(state) as f64);
    return ForcePair {
        first_force: Force {
            vertex: first.clone(),
            position_diff: scale_normalized_vector(to_normalized(Vector { first: second_position, second: first_position }), cnst),
        },
        second_force: Force {
            vertex: second.clone(),
            position_diff: scale_normalized_vector(to_normalized(Vector { first: first_position, second: second_position }), cnst),
        },
    };
}


fn apply_force_to_state(force: Force, state: &mut State) {
    state.positions.insert(force.vertex.clone(),
                           add(*state.positions.get(&force.vertex).unwrap(),
                               force.position_diff));
}

fn edge_forces_count(state: &State) -> usize {
    state.graph.edges.len() * 2
}

fn vertex_forces_count(state: &State) -> usize {
    state.graph.vertexes.len() * (state.graph.vertexes.len() - 1)
}