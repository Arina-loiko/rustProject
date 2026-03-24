use std::collections::HashMap;

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
}
