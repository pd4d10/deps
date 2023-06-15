use ast_grep_core::Language;
use ast_grep_language::Tsx;
use node_resolve::Resolver;
use petgraph::{graph::Graph, stable_graph::NodeIndex};
use std::{
    fs,
    io::BufRead,
    path::{Path, PathBuf},
    process::Command,
};

pub struct Walker {
    root: String,
    pub graph: Graph<String, u8>,
}

const ROOT_NODE: &str = "#ROOT";

impl Walker {
    pub fn new(root: String) -> Self {
        Walker {
            root,
            graph: Graph::new(),
        }
    }

    pub fn collect(&mut self, entry: &String, parent_node: NodeIndex) {
        let duplicated = self
            .graph
            .node_indices()
            .find(|i: &NodeIndex| self.graph[*i] == entry.to_owned());

        if duplicated.is_some() {
            return;
        }

        println!("[collecting] {entry}");

        // add to graph
        let current_node = self.graph.add_node(entry.to_owned());
        self.graph.add_edge(parent_node, current_node, 1);

        // by extension
        let ext = Path::new(entry).extension().unwrap().to_str().unwrap();

        let js_extensions = [".ts", ".tsx", ".js", ".jsx", ".json"];
        if !js_extensions.contains(&ext) {
            return;
        }

        let abs_path = Path::new(&self.root).join(entry.to_owned());
        let code = fs::read_to_string(abs_path).expect("Should have been able to read the file");

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
                        let entry = Path::new(resolved.to_str().unwrap())
                            .strip_prefix(PathBuf::from(&self.root))
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string();
                        self.collect(&entry, current_node);
                    }
                }
            });
    }

    pub fn collect_all(&mut self) {
        let output = Command::new("git")
            .current_dir(Path::new(&self.root))
            .arg("ls-files")
            .output()
            .expect("git list files fail");
        let files = output.stdout.lines().map(|str| str.unwrap());

        let root_node = self.graph.add_node(ROOT_NODE.to_string());
        files.for_each(|file| self.collect(&file, root_node))
    }
}
