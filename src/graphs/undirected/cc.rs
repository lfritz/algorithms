//! Algorithm 4.3: Depth-first search to find connected components in a graph.

use super::Graph;

pub struct ConnectedComponents {
    id: Vec<usize>,
    count: usize,
}

impl ConnectedComponents {

    /// Find connected components with depth-first search.
    pub fn find(g: &Graph) -> ConnectedComponents {
        let mut cc = ConnectedComponents {
            id: vec![0; g.v],
            count: 0,
        };
        let mut marked = vec![false; g.v];
        for v in 0..g.v {
            if ! marked[v] {
                cc.dfs(g, &mut marked, v);
                cc.count += 1;
            }
        }
        cc
    }

    fn dfs(&mut self, g: &Graph, marked: &mut Vec<bool>, v: usize) {
        marked[v] = true;
        self.id[v] = self.count;
        let neighbors = g.adj[v].clone();
        for w in neighbors {
            if ! marked[w] {
                self.dfs(g, marked, w);
            }
        }
    }

    /// Check if node `v` and node `w` in the graph are connected.
    pub fn connected(&self, v: usize, w: usize) -> bool {
        return self.id[v] == self.id[w];
    }

}

#[cfg(test)]
mod test {
    use super::ConnectedComponents;
    use super::super::Graph;
    use super::super::test::sample_graph;

    #[test]
    fn works_with_no_edges() {
        let g = Graph::new(3);
        let cc = ConnectedComponents::find(&g);
        assert_eq!(3, cc.count);
        assert!( ! cc.connected(0, 1));
        assert!( ! cc.connected(0, 2));
        assert!( ! cc.connected(1, 2));
    }

    #[test]
    fn works_for_complete_graph() {
        let mut g = Graph::new(3);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 2);
        let cc = ConnectedComponents::find(&g);
        assert_eq!(1, cc.count);
        assert!(cc.connected(0, 1));
        assert!(cc.connected(0, 2));
        assert!(cc.connected(1, 2));
    }

    #[test]
    fn works_for_sample_graph() {
        let g = sample_graph();
        let cc = ConnectedComponents::find(&g);
        assert_eq!(3, cc.count);
        assert!(cc.connected(1, 3));
        assert!(cc.connected(10, 12));
        assert!( ! cc.connected(1, 12)); 
    }

}