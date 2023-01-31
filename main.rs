mod force_driven_layout;
mod graph;

extern crate core;
use crate::graph::*;
use crate::force_driven_layout::State;
use crate::force_driven_layout::Position;
use crate::force_driven_layout::iterate;

fn main() {
    test();
}

fn test(){
    let state = &mut State {
        graph: Graph { vertexes: vec![Vertex { id: 1 },
                                      Vertex { id: 2 },
                                      Vertex { id: 3 },
                                      Vertex { id: 4 }],
            edges: vec![Edge{
                first: Vertex {id:1},
                second: Vertex {id:2}
            },
                        Edge{
                            first: Vertex {id:2},
                            second: Vertex {id:3}
                        },
                        Edge{
                            first: Vertex {id:3},
                            second: Vertex {id:4}
                        },
                        Edge{
                            first: Vertex {id:4},
                            second: Vertex {id:1}
                        }]
        },
        positions: Default::default(),
    };
    state.positions.insert(Vertex{id:1},Position{x:100,y:100});
    state.positions.insert(Vertex{id:2},Position{x:200,y:200});
    state.positions.insert(Vertex{id:3},Position{x:300,y:300});
    state.positions.insert(Vertex{id:4},Position{x:400,y:400});
    println!("WORK");
    display(state);
    iterate(state);
    display(state);
    iterate(state);
    display(state);
    iterate(state);
    display(state);
}

fn display(state:&State){
    println!();
    println!("1: {},{}", state.positions.get(&Vertex { id: 1 }).unwrap().x, state.positions.get(&Vertex { id: 1 }).unwrap().y);
    println!("2: {},{}", state.positions.get(&Vertex { id: 2 }).unwrap().x, state.positions.get(&Vertex { id: 2 }).unwrap().y);
    println!("3: {},{}", state.positions.get(&Vertex { id: 3 }).unwrap().x, state.positions.get(&Vertex { id: 3 }).unwrap().y);
    println!("4: {},{}", state.positions.get(&Vertex { id: 4 }).unwrap().x, state.positions.get(&Vertex { id: 4 }).unwrap().y);
}



