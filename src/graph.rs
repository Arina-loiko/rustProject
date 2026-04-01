use std::collections::{HashMap, VecDeque};

pub type NodeId = usize;

#[derive(Debug, Default)]
pub struct DepGraph {
    names: Vec<String>,
    index: HashMap<String, NodeId>,
    edges: Vec<Vec<NodeId>>,
}

impl DepGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, name: &str) -> NodeId {
        if let Some(&id) = self.index.get(name) {
            return id;
        }
        let id = self.names.len();
        self.names.push(name.to_string());
        self.index.insert(name.to_string(), id);
        self.edges.push(Vec::new());
        id
    }

    pub fn add_edge(&mut self, from: NodeId, to: NodeId) {
        if !self.edges[from].contains(&to) {
            self.edges[from].push(to);
        }
    }

    pub fn name(&self, id: NodeId) -> &str {
        &self.names[id]
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    pub fn find_cycle(&self) -> Option<Vec<String>> {
        let n = self.names.len();
        let mut color = vec![0u8; n];
        let mut path: Vec<NodeId> = Vec::new();
        for start in 0..n {
            if color[start] == 0 {
                if let Some(cycle) = self.dfs_cycle(start, &mut color, &mut path) {
                    return Some(cycle.into_iter().map(|id| self.names[id].clone()).collect());
                }
            }
        }
        None
    }

    pub fn topo_sort(&self) -> Option<Vec<NodeId>> {
        let n = self.names.len();
        let mut indeg = vec![0usize; n];
        for u in 0..n {
            for &v in &self.edges[u] {
                indeg[v] += 1;
            }
        }
        let mut queue: VecDeque<NodeId> = (0..n).filter(|&i| indeg[i] == 0).collect();
        let mut order = Vec::with_capacity(n);
        while let Some(u) = queue.pop_front() {
            order.push(u);
            for &v in &self.edges[u] {
                indeg[v] -= 1;
                if indeg[v] == 0 {
                    queue.push_back(v);
                }
            }
        }
        if order.len() == n { Some(order) } else { None }
    }

    fn dfs_cycle(&self, u: NodeId, color: &mut [u8], path: &mut Vec<NodeId>) -> Option<Vec<NodeId>> {
        color[u] = 1;
        path.push(u);
        for &v in &self.edges[u] {
            if color[v] == 1 {
                let idx = path.iter().position(|&x| x == v).unwrap();
                let mut cycle: Vec<NodeId> = path[idx..].to_vec();
                cycle.push(v);
                return Some(cycle);
            } else if color[v] == 0 {
                if let Some(c) = self.dfs_cycle(v, color, path) {
                    return Some(c);
                }
            }
        }
        path.pop();
        color[u] = 2;
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_node_dedup() {
        let mut g = DepGraph::new();
        let a1 = g.add_node("a");
        let a2 = g.add_node("a");
        assert_eq!(a1, a2);
        assert_eq!(g.len(), 1);
    }

    #[test]
    fn add_edge_no_duplicates() {
        let mut g = DepGraph::new();
        let a = g.add_node("a");
        let b = g.add_node("b");
        g.add_edge(a, b);
        g.add_edge(a, b);
        assert_eq!(g.edges[a].len(), 1);
    }

    #[test]
    fn finds_simple_cycle() {
        let mut g = DepGraph::new();
        let a = g.add_node("a");
        let b = g.add_node("b");
        g.add_edge(a, b);
        g.add_edge(b, a);
        let cycle = g.find_cycle().expect("должен быть цикл");
        assert_eq!(cycle.first(), cycle.last());
        assert!(cycle.contains(&"a".to_string()));
        assert!(cycle.contains(&"b".to_string()));
    }

    #[test]
    fn no_cycle_in_dag() {
        let mut g = DepGraph::new();
        let a = g.add_node("a");
        let b = g.add_node("b");
        let c = g.add_node("c");
        g.add_edge(a, b);
        g.add_edge(b, c);
        assert!(g.find_cycle().is_none());
    }

    #[test]
    fn topo_sort_linear() {
        let mut g = DepGraph::new();
        let a = g.add_node("a");
        let b = g.add_node("b");
        let c = g.add_node("c");
        g.add_edge(a, b);
        g.add_edge(b, c);
        let order = g.topo_sort().unwrap();
        let pos: HashMap<_, _> = order.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        assert!(pos[&a] < pos[&b]);
        assert!(pos[&b] < pos[&c]);
    }

    #[test]
    fn topo_sort_diamond() {
        let mut g = DepGraph::new();
        let a = g.add_node("a");
        let b = g.add_node("b");
        let c = g.add_node("c");
        let d = g.add_node("d");
        g.add_edge(a, b);
        g.add_edge(a, c);
        g.add_edge(b, d);
        g.add_edge(c, d);
        let order = g.topo_sort().unwrap();
        let pos: HashMap<_, _> = order.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        assert!(pos[&a] < pos[&b]);
        assert!(pos[&a] < pos[&c]);
        assert!(pos[&b] < pos[&d]);
        assert!(pos[&c] < pos[&d]);
    }

    #[test]
    fn topo_sort_returns_none_on_cycle() {
        let mut g = DepGraph::new();
        let a = g.add_node("a");
        let b = g.add_node("b");
        g.add_edge(a, b);
        g.add_edge(b, a);
        assert!(g.topo_sort().is_none());
    }
}
