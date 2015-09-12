//! Algorithm 4.1: Depth-first search to find paths in an undirected graph.

use super::Graph;

pub struct DepthFirstSearch {
    marked: Vec<bool>,
    edge_to: Vec<usize>,
    s: usize,
}

impl DepthFirstSearch {

    /// Depth-first search to find paths from `s` to nodes connected to `s`.
    pub fn new(g: &Graph, s: usize) -> DepthFirstSearch {
        let mut d = DepthFirstSearch {
            marked: vec![false; g.v],
            edge_to: vec![0; g.v],
            s: s,
        };
        d.dfs(g, s);
        d
    }

    fn dfs(&mut self, g: &Graph, v: usize) {
        self.marked[v] = true;
        let neighbors = g.adj[v].clone();
        for w in neighbors {
            if !self.marked[w] {
                self.edge_to[w] = v;
                self.dfs(g, w);
            }
        }
    }

    /// Return the path from `s` to `v` found by depth-first search.
    pub fn path_to(&self, v: usize) -> Vec<usize> {
        let mut path = Vec::new();
        let mut x = v;
        path.push(x);
        while x != self.s {
            x = self.edge_to[x];
            path.push(x);
        }
        path.reverse();
        path
    }

}

#[cfg(test)]
mod test {
    use super::DepthFirstSearch;
    use super::super::test::sample_graph;

    #[test]
    fn s() {
        let g = sample_graph();
        let d = DepthFirstSearch::new(&g, 7);
        assert_eq!(7, d.s);
    }

    #[test]
    fn marked() {
        let g = sample_graph();
        let d = DepthFirstSearch::new(&g, 7);
        assert_eq!(vec![false, false, false, false, false, false, false,
                        true, true, false, false, false, false],
                   d.marked);
    }

    #[test]
    fn edge_to() {
        let g = sample_graph();
        let d = DepthFirstSearch::new(&g, 0);
        assert_eq!(0, d.edge_to[1]);
        assert_eq!(0, d.edge_to[2]);
        assert!(d.edge_to[3] == 4 || d.edge_to[3] == 5);
        assert!(d.edge_to[4] == 3 || d.edge_to[4] == 5 || d.edge_to[4] == 6);
        assert!(d.edge_to[5] == 0 || d.edge_to[5] == 3 || d.edge_to[5] == 4);
        assert!(d.edge_to[6] == 0 || d.edge_to[6] == 4);
    }

    #[test]
    fn path_to() {
        let g = sample_graph();
        let d = DepthFirstSearch::new(&g, 0);
        assert_eq!(vec![0,1], d.path_to(1));
        assert_eq!(vec![0,2], d.path_to(2));
        let p3 = d.path_to(3);
        assert!(p3 == vec![0,5,3]
             || p3 == vec![0,5,4,3]
             || p3 == vec![0,6,4,3]);
    }

}

