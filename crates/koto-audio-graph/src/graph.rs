//! Audio graph structure

use std::collections::HashMap;

/// Unique identifier for a node in the graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u64);

/// Connection between two nodes
#[derive(Debug, Clone, Copy)]
pub struct Connection {
    pub source: NodeId,
    pub source_port: u32,
    pub target: NodeId,
    pub target_port: u32,
}

/// Audio graph structure
pub struct AudioGraph {
    nodes: HashMap<NodeId, Box<dyn AudioNode>>,
    connections: Vec<Connection>,
    next_id: u64,
}

impl AudioGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            connections: Vec::new(),
            next_id: 0,
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: Box<dyn AudioNode>) -> NodeId {
        let id = NodeId(self.next_id);
        self.next_id += 1;
        self.nodes.insert(id, node);
        id
    }

    /// Remove a node from the graph
    pub fn remove_node(&mut self, id: NodeId) {
        self.nodes.remove(&id);
        self.connections
            .retain(|c| c.source != id && c.target != id);
    }

    /// Connect two nodes
    pub fn connect(&mut self, connection: Connection) {
        self.connections.push(connection);
    }

    /// Disconnect two nodes
    pub fn disconnect(&mut self, source: NodeId, target: NodeId) {
        self.connections
            .retain(|c| c.source != source || c.target != target);
    }

    /// Get a node by ID
    pub fn get_node(&self, id: NodeId) -> Option<&dyn AudioNode> {
        self.nodes.get(&id).map(|n| n.as_ref())
    }

    /// Get a mutable reference to a node by ID
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut dyn AudioNode> {
        self.nodes.get_mut(&id).map(|n| n.as_mut())
    }
}

impl Default for AudioGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for audio processing nodes in the graph
pub trait AudioNode: Send + 'static {
    /// Get the number of input ports
    fn input_count(&self) -> usize;

    /// Get the number of output ports
    fn output_count(&self) -> usize;

    /// Process audio (placeholder - actual implementation in audio-engine)
    fn name(&self) -> &str;
}
