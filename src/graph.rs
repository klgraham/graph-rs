
use std::collections::HashMap;


// Adjacency list graph
#[derive(Debug)]
struct Vertex {
    pub id: usize,
    pub string_props: HashMap<String, String>,
    pub int_props: HashMap<String, i64>,
    pub float_props: HashMap<String, f64>
}

// struct Edge {
//     pub weight: f64,
//     pub source: u64,
//     pub sink: u64,
//     pub string_props: HashMap<String, String>,
//     pub int_props: HashMap<String, i64>,
//     pub float_props: HashMap<String, f64>
// }

#[derive(Debug)]
struct Graph {
    pub is_directed: bool,
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Vec<usize>>
}

impl Vertex {
    pub fn new(id: usize) -> Vertex {
        Vertex {
            id,
            string_props: HashMap::new(),
            int_props: HashMap::new(),
            float_props: HashMap::new()
        }
    }

    pub fn add_string_props(&mut self, label: String, value: String) {
        self.string_props.insert(label, value);
    }

    pub fn add_int_props(&mut self, label: String, value: i64) {
        self.int_props.insert(label, value);
    }

    pub fn add_float_props(&mut self, label: String, value: f64) {
        self.float_props.insert(label, value);
    }
}

impl Graph {
    pub fn new(is_directed: bool) -> Graph {
        Graph {
            is_directed,
            vertices: vec![],
            edges: vec![]
        }
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn num_edges(&self) -> usize {
        self.edges.iter().map(|v| v.len()).sum()
    }

    pub fn get_vertex(&self, id: usize) -> Option<&Vertex> {
        self.vertices.get(id)
    }

    pub fn get_adjacent_vertices(&self, id: usize) -> Option<&Vec<usize>> {
        self.edges.get(id)
    }

    pub fn add_vertex(&mut self) -> usize {
        let n = self.num_vertices();
        let v = Vertex::new(n);
        self.vertices.push(v);
        // add empty adjacency list
        self.edges.push(Vec::new());
        return n
    }

    pub fn add_edge(&mut self, from: usize, to:usize) {
        self.edges[from].push(to);

        if !self.is_directed {
            self.edges[to].push(from);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{Vertex, Graph};

    #[test]
    fn can_make_vertices() {
        let v = Vertex::new(5);
        assert_eq!(v.id, 5);
    }

    #[test]
    fn can_add_vertices() {
        let mut g = Graph::new(false);
        let id = g.add_vertex();
        let v = g.get_vertex(id).unwrap();
        assert_eq!(g.num_vertices(), 1);
        assert_eq!(id, v.id);
        assert!(g.edges[id].is_empty());
    }

    #[test]
    fn can_add_undirected_edges() {
        let mut undirected = Graph::new(false);
        let v1 = undirected.add_vertex();
        let v2 = undirected.add_vertex();
        undirected.add_edge(v1, v2);
        assert_eq!(undirected.num_vertices(), 2);
        assert_eq!(undirected.num_edges(), 2);
        assert_eq!(undirected.get_vertex(v2).unwrap().id, v2);
        assert_eq!(undirected.edges[v1][0], v2);
        assert_eq!(undirected.edges[v2][0], v1);
    }

    #[test]
    fn can_add_directed_edges() {
        let mut directed = Graph::new(true);
        let v1 = directed.add_vertex();
        let v2 = directed.add_vertex();
        directed.add_edge(v1, v2);
        assert_eq!(directed.num_vertices(), 2);
        assert_eq!(directed.num_edges(), 1);
        assert_eq!(directed.get_vertex(v2).unwrap().id, v2);
        assert_eq!(directed.edges[v1][0], v2);
        assert!(directed.edges[v2].is_empty());
    }
}