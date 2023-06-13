use ast_grep_core::Language;
use ast_grep_language::Tsx;
use clap::Parser;
use std::fs;

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
    find_deps(args.path)
}

fn find_deps(path: String) {
    let code = fs::read_to_string(path).expect("Should have been able to read the file");
    let sg = Tsx.ast_grep(code);

    let nodes = sg.root().find_all("import $$$_ from \"$P\"");

    println!("Found {}", nodes.count());
}
