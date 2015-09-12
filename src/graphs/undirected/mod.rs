//! Implementations of data structures and algorithms from Chapter 4.1:
//! Undirected Graphs.

pub mod bfs;
pub mod dfs;

/// Data structure for an undirected graph.
pub struct Graph {
    /// number of vertices
    v: usize,
    /// number of edges
    e: usize,
    /// adjacency lists
    adj: Vec<Vec<usize>>,
}

impl Graph {
    /// Constructs a new, empty graph.
    pub fn new(v: usize) -> Graph {
        Graph { v: v, e: 0, adj: vec![vec![]; v] }
    }

    /// Add edge v-w to the graph.
    pub fn add_edge(&mut self, v: usize, w: usize) {
        assert!(v < self.v);
        assert!(w < self.v);
        self.adj[v].push(w);
        self.adj[w].push(v);
        self.e += 1;
    }
}

#[cfg(test)]
mod test {
    use super::Graph;

    /// Sample graph from the book (page 529).
    pub fn sample_graph() -> Graph {
        let mut g = Graph::new(13);
        g.add_edge(0, 5);
        g.add_edge(4, 3);
        g.add_edge(0, 1);
        g.add_edge(9, 12);
        g.add_edge(6, 4);
        g.add_edge(5, 4);
        g.add_edge(0, 2);
        g.add_edge(11, 12);
        g.add_edge(9, 10);
        g.add_edge(0, 6);
        g.add_edge(7, 8);
        g.add_edge(9, 11);
        g.add_edge(5, 3);
        g
    }

    #[test]
    fn empty_graph() {
        let g = Graph::new(0);
        assert_eq!(0, g.v);
        assert_eq!(0, g.e);
    }

    #[test]
    fn no_edges() {
        let g = Graph::new(5);
        assert_eq!(5, g.v);
        assert_eq!(0, g.e);
    }

    #[test]
    fn sample() {
        let g = sample_graph();
        assert_eq!(13, g.v);
        assert_eq!(13, g.e);
    }
}

