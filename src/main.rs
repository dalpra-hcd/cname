use tree_sitter::Parser;

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
    let mut parser = Parser::new();
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
