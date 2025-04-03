use emmylua_parser::{BinaryOperator, LuaAstNode, LuaBlock, LuaDocTagCast};

use crate::{
    compilation::analyzer::AnalyzeContext, FileId, InFiled, LuaFlowChain, LuaType, TypeAssertion,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CastAction {
    Force,
    Add,
    Remove,
}

pub fn analyze_cast(
    flow_chain: &mut LuaFlowChain,
    file_id: FileId,
    name: &str,
    tag: LuaDocTagCast,
    context: &AnalyzeContext,
) -> Option<()> {
    let effect_range = tag.ancestors::<LuaBlock>().next()?.get_range();
    let actual_range = tag.get_range();
    for cast_op_type in tag.get_op_types() {
        let action = match cast_op_type.get_op() {
            Some(op) => {
                if op.get_op() == BinaryOperator::OpAdd {
                    CastAction::Add
                } else {
                    CastAction::Remove
                }
            }
            None => CastAction::Force,
        };

        if cast_op_type.is_nullable() {
            match action {
                CastAction::Add => {
                    flow_chain.add_type_assert(
                        name,
                        TypeAssertion::Add(LuaType::Nil),
                        effect_range,
                        actual_range,
                    );
                }
                CastAction::Remove => {
                    flow_chain.add_type_assert(
                        name,
                        TypeAssertion::Remove(LuaType::Nil),
                        effect_range,
                        actual_range,
                    );
                }
                _ => {}
            }
        } else if let Some(doc_typ) = cast_op_type.get_type() {
            let key = InFiled::new(file_id, doc_typ.get_syntax_id());
            let typ = match context.cast_flow.get(&key) {
                Some(t) => t.clone(),
                None => continue,
            };

            match action {
                CastAction::Add => {
                    flow_chain.add_type_assert(
                        name,
                        TypeAssertion::Add(typ),
                        effect_range,
                        actual_range,
                    );
                }
                CastAction::Remove => {
                    flow_chain.add_type_assert(
                        name,
                        TypeAssertion::Remove(typ),
                        effect_range,
                        actual_range,
                    );
                }
                CastAction::Force => {
                    flow_chain.add_type_assert(
                        name,
                        TypeAssertion::Narrow(typ),
                        effect_range,
                        actual_range,
                    );
                }
            }
        }
    }

    Some(())
}
