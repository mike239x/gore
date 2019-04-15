pub mod graph {
    pub trait Node {
        //TODO
    }
    pub trait Edge {
        //TODO
    }
    pub fn optimize(mut nodes: Vec<&Node>, edges: Vec<&Edge>) {
    }
    //pub trait Serializable {}
    //pub fn store(Vec<&Serializable>) {
    //}
    //pub fn load(mut Vec<&Serializable>) {
    //}
}

#[cfg(test)]
mod tests {
    #[test]
    fn optimize() {
        use super::graph;
        struct N;
        impl graph::Node for N {};
        struct E;
        impl graph::Edge for E {};
        let nodes = vec![N,N,N];
        let edges = vec![E,E];
        graph::optimize(&nodes, &edges);
        graph::optimize(&nodes, &edges);
    }
}
