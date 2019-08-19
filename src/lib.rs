#[allow(dead_code)]
struct Node;

#[allow(dead_code)]
struct Edge;

#[allow(dead_code)]
struct Nodes;

#[allow(dead_code)]
struct Edges;

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
        break;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_optimize() {
        // use super::{ };
    }
}
