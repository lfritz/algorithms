//! Algorithm 4.2: Breadth-first search to find paths in an undirected graph.

use super::Graph;

pub struct BreadthFirstSearch {
    marked: Vec<bool>,
    edge_to: Vec<usize>,
    s: usize,
}

impl BreadthFirstSearch {

    /// Breadth-first search to find paths from `s` to nodes connected to `s`.
    pub fn new(g: &Graph, s: usize) -> BreadthFirstSearch {
        let mut d = BreadthFirstSearch {
            marked: vec![false; g.v],
            edge_to: vec![0; g.v],
            s: s,
        };
        d.bfs(g, s);
        d
    }

    fn bfs(&mut self, g: &Graph, s: usize) {
        let mut queue = Vec::new();
        self.marked[s] = true;
        queue.push(s);
        while !queue.is_empty() {
            let v = queue.remove(0);
            let neighbors = g.adj[v].clone();
            for w in neighbors {
                if !self.marked[w] {
                    self.edge_to[w] = v;
                    self.marked[w] = true;
                    queue.push(w);
                }
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
    use super::BreadthFirstSearch;
    use super::super::test::sample_graph;

    #[test]
    fn s() {
        let g = sample_graph();
        let d = BreadthFirstSearch::new(&g, 7);
        assert_eq!(7, d.s);
    }

    #[test]
    fn marked() {
        let g = sample_graph();
        let d = BreadthFirstSearch::new(&g, 7);
        assert_eq!(vec![false, false, false, false, false, false, false,
                        true, true, false, false, false, false],
                   d.marked);
    }

    #[test]
    fn edge_to() {
        let g = sample_graph();
        let d = BreadthFirstSearch::new(&g, 0);
        assert_eq!(0, d.edge_to[1]);
        assert_eq!(0, d.edge_to[2]);
        assert!(d.edge_to[3] == 5);
        assert!(d.edge_to[4] == 5 || d.edge_to[4] == 6);
        assert!(d.edge_to[5] == 0);
        assert!(d.edge_to[6] == 0);
    }

    #[test]
    fn path_to() {
        let g = sample_graph();
        let d = BreadthFirstSearch::new(&g, 0);
        assert_eq!(vec![0,1], d.path_to(1));
        assert_eq!(vec![0,2], d.path_to(2));
        let p3 = d.path_to(3);
        assert!(p3 == vec![0,5,3]);
    }

}

