pub struct HalfEdge {
    pub twin: Option<HalfEdge>,
    pub next: Option<HalfEdge>,
    pub prev: Option<HalfEdge>,
    pub origin: Option<Vertex>,
}

pub struct Vertex {
    pub position: [f32; 3],
    pub half_edge: Option<HalfEdge>,
}

pub struct Face {
    pub half_edge: Option<HalfEdge>,
}

pub struct Edge {
    pub half_edge: Option<HalfEdge>,
}
