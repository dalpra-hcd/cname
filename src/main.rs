use tree_sitter::{InputEdit, Language, Parser, Point};



fn main() {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_rust::LANGUAGE.into()).expect("Error loading Rust grammar");

    let source_code = "fn test() {}";
    let mut tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();

    println!("{}", root_node.kind());
    println!("{}", root_node.start_position().column);
    println!("{}", root_node.end_position().column);
}
