use petgraph::{visit::EdgeRef, Direction, Graph};

pub struct Printer {
    graph: Graph<String, u8>,
}

impl Printer {
    pub fn new(graph: Graph<String, u8>) -> Self {
        Printer { graph }
    }

    pub fn print(&self, entry: String, indent: u8) {
        let node = self.graph.node_indices().find(|i| self.graph[*i] == entry);

        if node.is_none() {
            return;
        }

        println!("{}", entry);

        self.graph
            .edges_directed(node.unwrap(), Direction::Incoming)
            .for_each(|edge| {
                let path = self.graph.node_weight(edge.source()).unwrap();
                self.print(path.to_owned(), indent + 2);
            })
    }
}
