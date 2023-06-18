use clap::Parser;
use dls::{Printer, Walker};

/// Dependency list
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File path
    #[arg(short, long)]
    root: String,

    #[arg(short, long)]
    entry: String,
}

fn main() {
    let args = Args::parse();
    let mut walker = Walker::new(args.root);
    walker.collect_all();

    let printer = Printer::new(walker.graph);
    printer.print(args.entry, 0);
}
