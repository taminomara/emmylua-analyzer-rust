#[cfg(test)]
mod test {
    use smol_str::SmolStr;

    use crate::{LuaType, LuaUnionType, VirtualWorkspace};

    #[test]
    fn test_issue_318() {
        let mut ws = VirtualWorkspace::new_with_init_std_lib();

        ws.def(
            r#"
        local map = {
            a = 'a',
            b = 'b',
            c = 'c',
        }
        local key      --- @type string
        c = map[key]   -- type should be ('a'|'b'|'c'|nil)

        "#,
        );

        let c_ty = ws.expr_ty("c");

        let union_type = LuaType::Union(
            LuaUnionType::new(vec![
                LuaType::StringConst(SmolStr::new("a").into()),
                LuaType::StringConst(SmolStr::new("b").into()),
                LuaType::StringConst(SmolStr::new("c").into()),
                LuaType::Nil,
            ])
            .into(),
        );

        assert_eq!(c_ty, union_type);
    }
}
