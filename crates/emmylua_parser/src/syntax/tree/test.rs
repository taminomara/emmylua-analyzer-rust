#[cfg(test)]
mod test {
    use crate::{LineIndex, LuaAstNode, LuaParser, ParserConfig};
    use std::time::Instant;
    use std::{fs, thread};

    #[test]
    fn test_multithreaded_syntax_tree_traversal() {
        let code = r#"
            local a = 1
            local b = 2
            print(a + b)
        "#;
        let tree = LuaParser::parse(code, ParserConfig::default());
        let tree_arc = std::sync::Arc::new(tree);

        let mut handles = vec![];

        for i in 0..4 {
            let tree_ref = tree_arc.clone();
            let handle = thread::spawn(move || {
                let node = tree_ref.get_chunk_node();
                println!("{:?} {}", node.dump(), i);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
