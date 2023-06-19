use petgraph::{visit::EdgeRef, Direction, Graph};
use std::collections::HashSet;

pub struct Printer {
    printed: HashSet<String>,
    graph: Graph<String, u8>,
}

impl Printer {
    pub fn new(graph: Graph<String, u8>) -> Self {
        Printer {
            printed: HashSet::new(),
            graph,
        }
    }

    pub fn print(&mut self, entry: String, indent: usize) {
        let node = self.graph.node_indices().find(|i| self.graph[*i] == entry);
        if node.is_none() {
            return;
        }

        let circular = !self.printed.insert(entry.to_owned());
        println!(
            "{:indent$}{}{}",
            "",
            entry,
            if circular { " [circular]" } else { "" }
        );

        if !circular {
            self.graph
                .clone()
                .edges_directed(node.unwrap(), Direction::Incoming)
                .for_each(|edge| {
                    let path = self.graph.node_weight(edge.source()).unwrap();
                    self.print(path.to_owned(), indent + 2);
                })
        }
    }
}
