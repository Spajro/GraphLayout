use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use crate::graph::*;

pub fn input(file_path: String) -> Result<Graph, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut id = 0;
    let mut vertexes: HashMap<String, Vertex> = HashMap::new();
    let mut edges: Vec<Edge> = Vec::new();
    rdr.records().for_each(|record| {
        let rec = record.unwrap();
        let s1 = rec.get(0).unwrap().to_string().clone();
        let s2 = rec.get(1).unwrap().to_string();
        let vertex1 = if vertexes.contains_key(&s1) {
            vertexes.get(&s1).unwrap().clone()
        } else {
            id += 1;
            let vertex = Vertex {
                id: id,
                label: s1.clone(),
            };
            vertexes.insert(s1, vertex.clone());
            vertex
        };
        let vertex2 = if vertexes.contains_key(&s2) {
            vertexes.get(&s2).unwrap().clone()
        } else {
            id += 1;
            let vertex = Vertex {
                id: id,
                label: s2.clone(),
            };
            vertexes.insert(s2, vertex.clone());
            vertex
        };
        edges.push(Edge { first: vertex1.clone(), second: vertex2.clone() });
    });

    Ok(Graph {
        vertexes: vertexes.iter().map(|pair| pair.1.clone()).collect(),
        edges: edges,
    })
}
