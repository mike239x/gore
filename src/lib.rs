trait Node {
    const DIM: i32;
    fn wiggle(self: &Self, offset: Vec<f32>) -> Self;
}

trait Edge {
}

trait Graph {
    fn new() -> Self;
    fn add_node<N: Node>(self: &mut Self, node: N, String node_id) -> Self;
    fn add_edge<E: Edge>(self: &mut Self, edge: E, String edge_id) -> Self;
}

#[cfg(test)]
mod tests {
    #[test]
    fn optimize() {
        use super::{ Node, Edge };
        struct Pt(f32, f32); // (x, y)
        struct Line(f32, f32); // y = a * x + b;
        impl Node for Line {
            const DIM: i32 = 2;
            fn wiggle(self: &Line, offset: Vec<f32>) -> Line {
                return Line(self.0 + offset[0], self.1 + offset[1]);
            }
        }
        // impl Edge for (Line, Pt) {
        // }
        let _l = Line(0.0, 0.0);
        let _p0 = Pt(0.0, 0.0);
        let _p1 = Pt(1.0, 2.1);
        let _p2 = Pt(2.0, 3.9);
        let _p3 = Pt(3.0, 6.0);
    }
}
