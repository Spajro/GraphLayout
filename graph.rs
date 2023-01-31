#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct Vertex {
    pub(crate) id: u32,
}

#[derive(Copy, Clone)]
pub struct Edge {
    pub(crate) first: Vertex,
    pub(crate) second: Vertex,
}

#[derive(Clone)]
pub struct Graph {
    pub(crate) vertexes: Vec<Vertex>,
    pub(crate) edges: Vec<Edge>,
}