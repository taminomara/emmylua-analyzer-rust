use code_analysis::SemanticModel;
use emmylua_parser::{LuaAstNode, LuaSyntaxToken, LuaTokenKind};
use lsp_types::{SemanticToken, SemanticTokenType};
use rowan::NodeOrToken;

use crate::handlers::initialized::ClientId;

use super::{
    semantic_token_builder::SemanticBuilder, SEMANTIC_TOKEN_MODIFIERS, SEMANTIC_TOKEN_TYPES,
};

pub fn build_semantic_tokens(
    semantic_model: &mut SemanticModel,
    support_muliline_token: bool,
    client_id: ClientId,
) -> Option<Vec<SemanticToken>> {
    let root = semantic_model.get_root();
    let document = semantic_model.get_document();
    let mut builder = SemanticBuilder::new(
        &document,
        support_muliline_token,
        SEMANTIC_TOKEN_TYPES.to_vec(),
        SEMANTIC_TOKEN_MODIFIERS.to_vec(),
    );

    for node_or_token in root.syntax().descendants_with_tokens() {
        match node_or_token {
            NodeOrToken::Node(_) => {}
            NodeOrToken::Token(token) => {
                build_tokens_semantic_token(&mut builder, token, client_id);
            }
        }
    }

    Some(builder.build())
}

fn build_tokens_semantic_token(
    builder: &mut SemanticBuilder,
    token: LuaSyntaxToken,
    client_id: ClientId,
) {
    match token.kind().into() {
        LuaTokenKind::TkLongString | LuaTokenKind::TkString => {
            builder.push(token, SemanticTokenType::STRING);
        }
        LuaTokenKind::TkAnd
        | LuaTokenKind::TkBreak
        | LuaTokenKind::TkDo
        | LuaTokenKind::TkElse
        | LuaTokenKind::TkElseIf
        | LuaTokenKind::TkEnd
        | LuaTokenKind::TkFor
        | LuaTokenKind::TkFunction
        | LuaTokenKind::TkGoto
        | LuaTokenKind::TkIf
        | LuaTokenKind::TkIn
        | LuaTokenKind::TkNot
        | LuaTokenKind::TkOr
        | LuaTokenKind::TkRepeat
        | LuaTokenKind::TkReturn
        | LuaTokenKind::TkThen
        | LuaTokenKind::TkUntil
        | LuaTokenKind::TkWhile => {
            builder.push(token, SemanticTokenType::KEYWORD);
        }
        LuaTokenKind::TkLocal => {
            if !client_id.is_vscode() {
                builder.push(token, SemanticTokenType::KEYWORD);
            }
        }
        LuaTokenKind::TkPlus
        | LuaTokenKind::TkMinus
        | LuaTokenKind::TkMul
        | LuaTokenKind::TkDiv
        | LuaTokenKind::TkIDiv
        | LuaTokenKind::TkDot
        | LuaTokenKind::TkConcat
        | LuaTokenKind::TkEq
        | LuaTokenKind::TkGe
        | LuaTokenKind::TkLe
        | LuaTokenKind::TkNe
        | LuaTokenKind::TkShl
        | LuaTokenKind::TkShr
        | LuaTokenKind::TkLt
        | LuaTokenKind::TkGt
        | LuaTokenKind::TkMod
        | LuaTokenKind::TkPow
        | LuaTokenKind::TkLen
        | LuaTokenKind::TkBitAnd
        | LuaTokenKind::TkBitOr
        | LuaTokenKind::TkBitXor => {
            builder.push(token, SemanticTokenType::OPERATOR);
        }
        LuaTokenKind::TkComplex | LuaTokenKind::TkInt | LuaTokenKind::TkFloat => {
            builder.push(token, SemanticTokenType::NUMBER);
        }
        LuaTokenKind::TkTagClass
        | LuaTokenKind::TkTagEnum
        | LuaTokenKind::TkTagInterface
        | LuaTokenKind::TkTagAlias
        | LuaTokenKind::TkTagModule
        | LuaTokenKind::TkTagField
        | LuaTokenKind::TkTagType
        | LuaTokenKind::TkTagParam
        | LuaTokenKind::TkTagReturn
        | LuaTokenKind::TkTagOverload
        | LuaTokenKind::TkTagGeneric
        | LuaTokenKind::TkTagSee
        | LuaTokenKind::TkTagDeprecated
        | LuaTokenKind::TkTagAsync
        | LuaTokenKind::TkTagCast
        | LuaTokenKind::TkTagOther
        | LuaTokenKind::TkTagVisibility
        | LuaTokenKind::TkTagReadonly
        | LuaTokenKind::TkTagDiagnostic
        | LuaTokenKind::TkTagMeta
        | LuaTokenKind::TkTagVersion
        | LuaTokenKind::TkTagAs
        | LuaTokenKind::TkTagNodiscard
        | LuaTokenKind::TkTagOperator
        | LuaTokenKind::TkTagMapping
        | LuaTokenKind::TkTagNamespace
        | LuaTokenKind::TkTagUsing
        | LuaTokenKind::TkTagSource => {
            builder.push(token, SemanticTokenType::DECORATOR);
        }
        LuaTokenKind::TkDocKeyOf
        | LuaTokenKind::TkDocExtends
        | LuaTokenKind::TkDocAs
        | LuaTokenKind::TkDocIn
        | LuaTokenKind::TkDocInfer => {
            builder.push(token, SemanticTokenType::KEYWORD);
        }
        LuaTokenKind::TkDocDetail => {
            builder.push(token, SemanticTokenType::COMMENT);
        }
        LuaTokenKind::TkDocQuestion => {
            builder.push(token, SemanticTokenType::OPERATOR);
        }
        LuaTokenKind::TkDocVisibility => {
            builder.push(token, SemanticTokenType::DECORATOR);
        }
        LuaTokenKind::TkDocVersionNumber => {
            builder.push(token, SemanticTokenType::NUMBER);
        }
        LuaTokenKind::TkStringTemplateType => {
            builder.push(token, SemanticTokenType::STRING);
        }
        LuaTokenKind::TkDocMatch | LuaTokenKind::TkDocBoolean => {
            builder.push(token, SemanticTokenType::KEYWORD);
        }
        LuaTokenKind::TKDocPath => {
            builder.push(token, SemanticTokenType::STRING);
        }
        LuaTokenKind::TkDocRegion | LuaTokenKind::TkDocEndRegion => {
            builder.push(token, SemanticTokenType::KEYWORD);
        }
        _ => {}
    }
}
