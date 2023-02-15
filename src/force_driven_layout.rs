use std::collections::{HashMap, LinkedList};
use crate::config::Config;
use crate::graph::*;
use crate::vectors::*;

#[derive(Clone)]
pub struct State {
    pub graph: Graph,
    pub positions: HashMap<Vertex, Position>,
    pub config: Config,
}

#[derive(Clone)]
struct Force {
    vertex: Vertex,
    vector: StandardVector,
}

#[derive(Clone)]
struct ForcePair {
    first: Force,
    second: Force,
}

static EDGE_FORCE_LENGTH: f64 = 100.0;
static OPTIMAL_DISTANCE: f64 = 200.0;
static GRAPH_FORCE_FACTOR: f64 = EDGE_FORCE_LENGTH * OPTIMAL_DISTANCE;

pub fn iterate(state: &mut State) {
    let mut force_pairs = LinkedList::new();
    let mut forces = LinkedList::new();
    let mut final_forces_for_vertexes: HashMap<Vertex, StandardVector> = HashMap::new();

    state.graph.edges.iter()
        .for_each(|edge| force_pairs.push_back(edge_to_force_pair(edge, state)));

    state.graph.vertexes.iter()
        .for_each(|v1| state.graph.vertexes.iter()
            .for_each(|v2| force_pairs.push_back(vertexes_pair_to_force_pair(v1, v2, state))));

    force_pairs.iter()
        .for_each(|force_pair| {
            forces.push_back(force_pair.first.clone());
            forces.push_back(force_pair.second.clone());
        });

    state.graph.vertexes.iter()
        .for_each(|vertex| { final_forces_for_vertexes.insert(vertex.clone(), StandardVector { x: 0, y: 0 }); });

    forces.iter()
        .for_each(|force| {
            final_forces_for_vertexes.insert(force.vertex.clone(),
                                             final_forces_for_vertexes.get(&force.vertex).unwrap().add(force.vector));
        });

    final_forces_for_vertexes.iter().map(|pair| Force { vertex: pair.0.clone(), vector: *pair.1 })
        .for_each(|force| apply_force_to_state(force, state));
}

fn edge_to_force_pair(edge: &Edge, state: &State) -> ForcePair {
    let vector = Vector {
        first: state.positions.get(&edge.first).unwrap().clone(),
        second: state.positions.get(&edge.second).unwrap().clone(),
    };

    return ForcePair {
        first: Force {
            vertex: edge.first.clone(),
            vector: vector_to_force_vector(&vector, EDGE_FORCE_LENGTH),
        },
        second: Force {
            vertex: edge.second.clone(),
            vector: vector_to_force_vector(&vector.reversed(), EDGE_FORCE_LENGTH),
        },
    };
}

fn vertexes_pair_to_force_pair(first: &Vertex, second: &Vertex, state: &State) -> ForcePair {
    let vector = Vector {
        first: state.positions.get(first).unwrap().clone(),
        second: state.positions.get(second).unwrap().clone(),
    };

    let force_len: f64 = GRAPH_FORCE_FACTOR / vector.to_standard().len();

    return ForcePair {
        first: Force {
            vertex: first.clone(),
            vector: vector_to_force_vector(&vector.reversed(), force_len),
        },
        second: Force {
            vertex: second.clone(),
            vector: vector_to_force_vector(&vector, force_len),
        },
    };
}

fn vector_to_force_vector(vector: &Vector, new_length: f64) -> StandardVector {
    vector.to_standard().to_normalized().to_standard(new_length)
}

fn apply_force_to_state(force: Force, state: &mut State) {
    state.positions.insert(force.vertex.clone(),
                           state.positions.get(&force.vertex).unwrap().add_vector(force.vector));
}