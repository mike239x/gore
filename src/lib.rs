use std::iter::Iterator;
use std::collections::HashMap;
use std::sync::Arc;

#[allow(dead_code)]
type Key = String;

#[allow(dead_code)]
trait Node {
}

#[allow(dead_code)]
struct Edge {
    from : Key,
    to : Key,
}

#[allow(dead_code)]
type Nodes = HashMap<Key, Arc<dyn Node>>;
// TODO or mb Vec<Arc<Node>> ?
// also I probably need RwLock instead of Arc

#[allow(dead_code)]
struct Edges;

impl Iterator for Edges {
    type Item = Edge;
    fn next(&mut self) -> Option<Edge> {
        None
    }
}

#[allow(dead_code)]
/// graph has nodes and edges
struct Graph {
    nodes: Nodes,
    edges: Edges,
}

#[allow(dead_code)]
#[allow(unused_mut)]
#[allow(unused_variables)]
/// An optimisation graph is optimized as follows:
/// first, compute H and b
/// here the order of nodes can be chosen (so that later optimisation is computationally easier)
/// then solve H · x = -b with an arbitrary solver, such as Preconditioned Conjugate Gradients (PCG)
/// finally, update vertex data with an arbitrary optimization method (mostly GaussNewton / Levenberg-Marquardt)
/// rinse and repeat
fn optimize(mut g: Graph) {
    loop {
        for edge in g.edges {
            let Edge {from, to} = edge;
        }
        break;
    }
}

#[cfg(test)]
mod tests {
    mod nodes {
        #[test]
        fn does_not_change_ids_of_other_nodes_if_a_node_is_deleted() {
            // this looks dumb
        }
    }
    mod graph {
    #[test]
    fn can_be_optimized() {
        // use super::{ };
    }
    }
}
