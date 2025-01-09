#[cfg(test)]
mod test {
    use emmylua_parser::LuaSyntaxKind;
    use rowan::TextRange;

    use crate::{
        db_index::{LuaDecl, LuaDeclarationTree, LuaScopeKind},
        FileId,
    };

    fn create_decl_tree(file_id: FileId) -> LuaDeclarationTree {
        LuaDeclarationTree::new(file_id)
    }

    #[test]
    fn test_add_decl() {
        let file_id = FileId::new();
        let mut tree = create_decl_tree(file_id);
        let scope_id =
            tree.create_scope(TextRange::new(0.into(), 200.into()), LuaScopeKind::Normal);

        let decl = LuaDecl::Local {
            name: "aaa".to_string(),
            file_id,
            kind: LuaSyntaxKind::LocalName.into(),
            range: TextRange::new(5.into(), 8.into()),
            attrib: None,
            decl_type: None,
        };
        let decl_id = tree.add_decl(decl);

        tree.add_decl_to_scope(scope_id, decl_id.clone());

        let decl = tree.find_local_decl("aaa", 5.into());
        assert!(decl.is_none());

        let decl2 = tree.find_local_decl("aaa", 50.into());
        assert!(decl2.is_some());
        assert_eq!(decl2.unwrap().get_name(), "aaa");
        assert_eq!(decl2.unwrap().get_position(), 5.into());

        let decl3 = tree.find_local_decl("bbb", 9.into());
        assert!(decl3.is_none());
    }

    #[test]
    fn test_multi_scope() {
        let file_id = FileId::new();
        let mut tree = create_decl_tree(file_id);
        let scope_id1 =
            tree.create_scope(TextRange::new(0.into(), 200.into()), LuaScopeKind::Normal);
        let scope_id2 =
            tree.create_scope(TextRange::new(100.into(), 200.into()), LuaScopeKind::Normal);
        tree.add_child_scope(scope_id1, scope_id2);

        let decl = LuaDecl::Local {
            name: "aaa".to_string(),
            file_id,
            kind: LuaSyntaxKind::LocalName.into(),
            range: TextRange::new(5.into(), 8.into()),
            attrib: None,
            decl_type: None,
        };
        let decl_id = tree.add_decl(decl);

        tree.add_decl_to_scope(scope_id1, decl_id.clone());

        let decl2 = LuaDecl::Local {
            name: "bbb".to_string(),
            file_id,
            kind: LuaSyntaxKind::LocalName.into(),
            range: TextRange::new(105.into(), 108.into()),
            attrib: None,
            decl_type: None,
        };

        let decl_id2 = tree.add_decl(decl2);
        tree.add_decl_to_scope(scope_id2, decl_id2.clone());

        let decl = tree.find_local_decl("aaa", 5.into());
        assert!(decl.is_none());

        let decl2 = tree.find_local_decl("aaa", 50.into());
        assert!(decl2.is_some());
        assert_eq!(decl2.unwrap().get_name(), "aaa");
        assert_eq!(decl2.unwrap().get_position(), 5.into());

        let decl3 = tree.find_local_decl("aaa", 150.into());
        assert!(decl3.is_some());
        assert_eq!(decl3.unwrap().get_name(), "aaa");
        assert_eq!(decl3.unwrap().get_position(), 5.into());

        let decl4 = tree.find_local_decl("bbb", 150.into());
        assert!(decl4.is_some());
        assert_eq!(decl4.unwrap().get_name(), "bbb");
        assert_eq!(decl4.unwrap().get_position(), 105.into());

        let decl5 = tree.find_local_decl("bbb", 105.into());
        assert!(decl5.is_none());
    }

    #[test]
    fn test_global_decl() {
        let file_id = FileId::new();
        let mut tree = create_decl_tree(file_id);
        let scope_id1 =
            tree.create_scope(TextRange::new(0.into(), 200.into()), LuaScopeKind::Normal);
        let scope_id2 =
            tree.create_scope(TextRange::new(100.into(), 200.into()), LuaScopeKind::Normal);
        tree.add_child_scope(scope_id1, scope_id2);

        let decl = LuaDecl::Global {
            name: "aaa".to_string(),
            file_id,
            range: TextRange::new(5.into(), 8.into()),
            decl_type: None,
        };
        let decl_id = tree.add_decl(decl);

        tree.add_decl_to_scope(scope_id1, decl_id.clone());

        let decl = tree.find_local_decl("aaa", 5.into());
        assert!(decl.is_none());

        let decl2 = tree.find_local_decl("aaa", 50.into());
        assert!(decl2.is_some());
        assert_eq!(decl2.unwrap().get_name(), "aaa");
        assert_eq!(decl2.unwrap().get_position(), 5.into());

        let decl3 = tree.find_local_decl("aaa", 150.into());
        assert!(decl3.is_some());
        assert_eq!(decl3.unwrap().get_name(), "aaa");
        assert_eq!(decl3.unwrap().get_position(), 5.into());
    }

    #[test]
    fn test_same_decl_name() {
        let file_id = FileId::new();
        let mut tree = create_decl_tree(file_id);
        let scope_id1 =
            tree.create_scope(TextRange::new(0.into(), 200.into()), LuaScopeKind::Normal);
        let scope_id2 =
            tree.create_scope(TextRange::new(100.into(), 200.into()), LuaScopeKind::Normal);
        tree.add_child_scope(scope_id1, scope_id2);

        let decl = LuaDecl::Local {
            name: "aaa".to_string(),
            file_id,
            kind: LuaSyntaxKind::LocalName.into(),
            range: TextRange::new(5.into(), 8.into()),
            attrib: None,
            decl_type: None,
        };
        let decl_id = tree.add_decl(decl);

        tree.add_decl_to_scope(scope_id1, decl_id.clone());

        let decl2 = LuaDecl::Local {
            name: "aaa".to_string(),
            file_id,
            kind: LuaSyntaxKind::LocalName.into(),
            range: TextRange::new(105.into(), 108.into()),
            attrib: None,
            decl_type: None,
        };

        let decl_id2 = tree.add_decl(decl2);
        tree.add_decl_to_scope(scope_id2, decl_id2.clone());

        let decl = tree.find_local_decl("aaa", 5.into());
        assert!(decl.is_none());

        let decl2 = tree.find_local_decl("aaa", 50.into());
        assert!(decl2.is_some());
        assert_eq!(decl2.unwrap().get_name(), "aaa");
        assert_eq!(decl2.unwrap().get_position(), 5.into());

        let decl3 = tree.find_local_decl("aaa", 150.into());
        assert!(decl3.is_some());
        assert_eq!(decl3.unwrap().get_name(), "aaa");
        assert_eq!(decl3.unwrap().get_position(), 105.into());
    }

    #[test]
    fn test_repeat_scope() {
        let file_id = FileId::new();
        let mut tree = create_decl_tree(file_id);
        let root_scope_id =
            tree.create_scope(TextRange::new(0.into(), 200.into()), LuaScopeKind::Normal);
        let repeat_scope_id =
            tree.create_scope(TextRange::new(100.into(), 200.into()), LuaScopeKind::Repeat);
        let repeat_body_id =
            tree.create_scope(TextRange::new(110.into(), 150.into()), LuaScopeKind::Normal);

        tree.add_child_scope(root_scope_id, repeat_scope_id);
        tree.add_child_scope(repeat_scope_id, repeat_body_id);

        let decl = LuaDecl::Local {
            name: "aaa".to_string(),
            file_id,
            kind: LuaSyntaxKind::LocalName.into(),
            range: TextRange::new(5.into(), 8.into()),
            attrib: None,
            decl_type: None,
        };
        let decl_id = tree.add_decl(decl);

        tree.add_decl_to_scope(root_scope_id, decl_id.clone());

        let decl2 = LuaDecl::Local {
            name: "aaa".to_string(),
            file_id,
            kind: LuaSyntaxKind::LocalName.into(),
            range: TextRange::new(130.into(), 133.into()),
            attrib: None,
            decl_type: None,
        };

        let decl_id2 = tree.add_decl(decl2);
        tree.add_decl_to_scope(repeat_body_id, decl_id2.clone());

        let decl3 = LuaDecl::Local {
            name: "bbb".to_string(),
            file_id,
            kind: LuaSyntaxKind::LocalName.into(),
            range: TextRange::new(75.into(), 75.into()),
            attrib: None,
            decl_type: None,
        };

        let decl_id3 = tree.add_decl(decl3);
        tree.add_decl_to_scope(root_scope_id, decl_id3.clone());

        let decl = tree.find_local_decl("aaa", 5.into());
        assert!(decl.is_none());

        let decl2 = tree.find_local_decl("aaa", 50.into());
        assert!(decl2.is_some());
        assert_eq!(decl2.unwrap().get_name(), "aaa");
        assert_eq!(decl2.unwrap().get_position(), 5.into());

        let decl3 = tree.find_local_decl("aaa", 150.into());
        assert!(decl3.is_some());
        assert_eq!(decl3.unwrap().get_name(), "aaa");
        assert_eq!(decl3.unwrap().get_position(), 130.into());

        let decl4 = tree.find_local_decl("aaa", 175.into());
        assert!(decl4.is_some());
        assert_eq!(decl4.unwrap().get_name(), "aaa");
        assert_eq!(decl4.unwrap().get_position(), 130.into());

        let decl5 = tree.find_local_decl("bbb", 175.into());
        assert!(decl5.is_some());
        assert_eq!(decl5.unwrap().get_name(), "bbb");
        assert_eq!(decl5.unwrap().get_position(), 75.into());
    }
}
