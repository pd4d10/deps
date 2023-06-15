use ast_grep_core::Language;
use ast_grep_language::Tsx;
use clap::Parser;
use node_resolve::Resolver;
use petgraph::{graph::Graph, stable_graph::NodeIndex};
use std::{fs, path::PathBuf};

/// Dependency list
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File path
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();
    let mut walker = Walker::new(args.path);
    let entry = walker.entry.to_owned();
    let root_node = walker.graph.add_node(entry.to_owned());

    walker.collect(&entry, root_node);
    println!("Found {:?}", walker.graph);
}

struct Walker {
    // root: String,
    entry: String,
    graph: Graph<String, u8>,
}

impl Walker {
    fn new(entry: String) -> Self {
        Walker {
            entry,
            graph: Graph::new(),
        }
    }

    pub fn collect(&mut self, entry: &String, parent_node: NodeIndex) {
        if entry.contains("node_modules") {
            // TODO: use gitignore
            return;
        }

        println!("{entry}");

        let current_node = self.graph.add_node(entry.to_owned());
        self.graph.add_edge(parent_node, current_node, 1);

        let code = fs::read_to_string(entry).expect("Should have been able to read the file");
        let sg = Tsx.ast_grep(code);
        sg.root()
            .find_all("import $_ from \"$PATH\"")
            .for_each(|node| {
                let m = node.get_env().get_match("PATH");
                let specifier = m.unwrap().text().to_string();

                let resolver = Resolver::new()
                    .with_extensions([".ts", ".tsx", ".js", ".jsx", ".json"])
                    .with_main_fields(["source"]) // TODO:
                    .with_basedir(PathBuf::from(entry).parent().unwrap().to_owned());

                let resolved = resolver.resolve(&specifier);
                match resolved {
                    Err(_) => return,
                    Ok(resolved) => {
                        let entry = resolved.to_str().unwrap().to_string();
                        self.collect(&entry, current_node);
                    }
                }
            });
    }
}
