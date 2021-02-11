pub trait Edge: Clone {
    fn to(&self) -> usize;
}

pub struct Graph<E: Edge> {
    adj_list: Vec<Vec<E>>,
}

impl<E: Edge> Graph<E> {
    pub fn new(n: usize) -> Graph<E> {
        Graph {
            adj_list: vec![vec![]; n],
        }
    }

    pub fn len(&self) -> usize {
        self.adj_list.len()
    }

    pub fn add_edge(&mut self, from: usize, edge: E) {
        self.adj_list[from].push(edge);
    }

    pub fn edges(&self, from: usize) -> &[E] {
        &self.adj_list[from]
    }

    pub fn degree(&self, v: usize) -> usize {
        *&self.adj_list[v].len()
    }

    pub fn to_debug_str(&self) -> String {
        let n = self.len();
        let mut ret = String::new();
        ret.push_str("[\n");
        for v in 0..n {
            let s = self
                .edges(v)
                .iter()
                .map(|x| x.to().to_string())
                .collect::<Vec<_>>()
                .join(", ");
            ret.push_str(&format!("    [{}],\n", s));
        }
        ret.push_str("]");

        ret
    }
}

#[derive(Clone, Debug)]
pub struct SimpleEdge {
    to: usize,
}

impl SimpleEdge {
    pub fn new(to: usize) -> SimpleEdge {
        SimpleEdge { to }
    }
}

impl Edge for SimpleEdge {
    fn to(&self) -> usize {
        self.to
    }
}

pub type SimpleGraph = Graph<SimpleEdge>;

impl SimpleGraph {
    pub fn connect(&mut self, from: usize, to: usize) {
        self.add_edge(from, SimpleEdge::new(to));
    }
}
