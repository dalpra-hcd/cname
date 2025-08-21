use clap::Parser as ClapParser;
use serde::{Deserialize, Serialize};
use tree_sitter::Parser as TsParser;

#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn walk(node: tree_sitter::Node, source: &str) {
    if node.kind() == "identifier" {
        let name = node.utf8_text(source.as_bytes()).unwrap();
        println!("Found identifier: {name}");
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        walk(child, source);
    }
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }

    let point = Point { x: 1, y: 2 };

    let serialized = toml::to_string(&point).unwrap();
    println!("serialized = {serialized}");

    let deserialized: Point = toml::from_str(&serialized).unwrap();
    println!("deserialized = {deserialized:?}");

    let mut parser = TsParser::new();
    parser
        .set_language(&tree_sitter_c::LANGUAGE.into())
        .expect("Error loading C grammar");

    let source_code = r"
        int my_var = 0;
        void BadFunctionName() {}
    ";

    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();

    walk(root_node, source_code);
}
