use clap::Parser;
use dls::Walker;

/// Dependency list
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File path
    #[arg(short, long)]
    root: String,
}

fn main() {
    let args = Args::parse();
    let mut walker = Walker::new(args.root);
    walker.collect_all();
    println!("Found {:?}", walker.graph);
}
