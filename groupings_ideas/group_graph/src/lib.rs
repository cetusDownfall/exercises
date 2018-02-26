use std::cmp::{PartialEq, Eq, PartialOrd, Ord, Ordering};
pub mod graph {
    pub trait Graph<N, E>
        where N: Sized + PartialEq, E: Edge<Node = N> 
    {
        pub fn nodes(&self) -> Vec<N>;
        pub fn edge(&self, l: usize, r: usize) -> E;
        pub fn edges(&self, start: usize) -> Vec<E> {
            let mut out: Vec<E> = Vec::new();
            if start < self.nodes.len() {
                for n in 0..self.nodes.len() {
                    if n < start {
                        out.push(self.edge(n, start));
                    } else if n > start {
                        out.push(self.edge(start, n));
                    }
                }
            }
        }
        pub fn sorted_edges(&self, start: usize) -> Vec<E> {
            let mut out = self.edges(start);
            out.sort_unstable();
            out
        }
        pub fn least_edge(&self, start: usize) -> Option<E> {
            let mut edges = self.edges();
            if start < edges.len() {
                let mut out = edges.pop().unwrap();
                for e in edges.into_iter() {
                    if e < out {
                        out = e;
                    }
                }
                Some(out)
            } else {
                None
            }
        }
        pub fn node(&self, ind: usize) -> Option<N> {
            self.nodes().get(ind)
        }
        pub fn find_least_path(&self, start: usize) -> Vec<N> {
            let mut out: Vec<N> = Vec::new();
            let end_len = self.nodes.len();
            if start < end_len {
                let mut last_node = start;
                while out.len() < end_len {
                    let mut poss_edges = self.edges(last_node);
                    poss_edges.retain(|n| !out.contains(n));
                    poss_edges.sort_unstable();
                    out.push(last_node)
                    if let Some(n) = poss_edges.pop() {
                }
            }
        }
    }
}
pub mod path {
    pub trait Path {
        fn add_node(&mut self, 
    }
}
pub mod edge {
    pub trait Edge {
        type Node: Sized;
        fn l(&self) -> Rc<Node>,
        fn r(&self) -> Rc<Node>,
        fn w(&self) -> u32;
    }
    impl PartialEq for Edge {
        fn eq(&self, other: &Edge) -> bool {
            self.w() == other.w()
        }
    }
    impl PartialOrd for Edge {
        fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
            Some(self.w().cmp(other.w()))
        }
    }
    impl Eq for Edge {}
    impl Ord for Edge {
        fn cmp(&self, other: &Edge) -> Ordering {
            self.w().cmp(other.w())
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
