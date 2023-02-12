use std::collections::{HashMap, LinkedList};
use crate::config::Config;
use crate::graph::*;
use crate::vectors::*;

#[derive(Clone)]
pub struct State {
    pub(crate) graph: Graph,
    pub(crate) positions: HashMap<Vertex, Position>,
    pub(crate) config:Config,
}

#[derive(Clone)]
struct Force {
    vertex: Vertex,
    vector: StandardVector,
}

#[derive(Clone)]
struct ForcePair {
    first_force: Force,
    second_force: Force,
}

static EDGE_FORCE_LENGTH: f64 = 100.0;
static OPTIMAL_DISTANCE: f64 = 200.0;
static GRAPH_FORCE_FACTOR: f64 = EDGE_FORCE_LENGTH * OPTIMAL_DISTANCE;

pub fn iterate(state: &mut State) {
    let mut force_pairs = LinkedList::new();
    let mut forces = LinkedList::new();
    let mut joined: HashMap<Vertex, StandardVector> = HashMap::new();

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
        .for_each(|vertex| { joined.insert(vertex.clone(), StandardVector { x: 0, y: 0 }); });

    forces.iter()
        .for_each(|force| {
            joined.insert(force.vertex.clone(),
                          add_vectors(*joined.get(&force.vertex).unwrap(), force.vector));
        });

    joined.iter().map(|pair| Force { vertex: pair.0.clone(), vector: *pair.1 })
        .for_each(|force| apply_force_to_state(force, state));
}

fn edge_to_force_pair(edge: &Edge, state: &State) -> ForcePair {
    let first_position = *state.positions.get(&edge.first).unwrap();
    let second_position = *state.positions.get(&edge.second).unwrap();
    return ForcePair {
        first_force: Force {
            vertex: edge.first.clone(),
            vector: vector_to_force_vector(Vector { first: first_position, second: second_position }, EDGE_FORCE_LENGTH),
        },
        second_force: Force {
            vertex: edge.second.clone(),
            vector: vector_to_force_vector(Vector { first: second_position, second: first_position }, EDGE_FORCE_LENGTH),
        },
    };
}

fn vertexes_pair_to_force_pair(first: &Vertex, second: &Vertex, state: &State) -> ForcePair {
    let first_position = *state.positions.get(first).unwrap();
    let second_position = *state.positions.get(second).unwrap();
    let len: f64 = GRAPH_FORCE_FACTOR / standard_len(to_standard(Vector { first: second_position, second: first_position }));
    return ForcePair {
        first_force: Force {
            vertex: first.clone(),
            vector: vector_to_force_vector(Vector { first: second_position, second: first_position }, len),
        },
        second_force: Force {
            vertex: second.clone(),
            vector: vector_to_force_vector(Vector { first: first_position, second: second_position }, len),
        },
    };
}

fn vector_to_force_vector(vector: Vector, new_length: f64) -> StandardVector {
    normalized_to_standard(standard_to_normalized(to_standard(vector)), new_length)
}


fn apply_force_to_state(force: Force, state: &mut State) {
    state.positions.insert(force.vertex.clone(),
                           move_position_by_vector(*state.positions.get(&force.vertex).unwrap(), force.vector));
}