use code_analysis::{DbIndex, LuaPropertyOwnerId, LuaSignatureId, LuaType};

use code_analysis::humanize_type;
pub fn hover_const_type(db: &DbIndex, typ: &LuaType) -> String {
    let const_value = humanize_type(db, typ);

    match typ {
        LuaType::IntegerConst(_) | LuaType::DocIntegerConst(_) => {
            format!("integer = {}", const_value)
        }
        LuaType::FloatConst(_) => format!("number = {}", const_value),
        LuaType::StringConst(_) | LuaType::DocStringConst(_) => format!("string = {}", const_value),
        _ => const_value,
    }
}

pub fn hover_function_type(db: &DbIndex, typ: &LuaType, func_name: &str, is_local: bool) -> String {
    match typ {
        LuaType::Function => {
            format!(
                "{}function {}()",
                if is_local { "local " } else { "" },
                func_name
            )
        }
        LuaType::DocFunction(lua_func) => {
            let async_prev = if lua_func.is_async() { "async " } else { "" };
            let local_prev = if is_local { "local " } else { "" };
            let params = lua_func
                .get_params()
                .iter()
                .map(|param| {
                    let name = param.0.clone();
                    if let Some(ty) = &param.1 {
                        format!("{}: {}", name, humanize_type(db, ty))
                    } else {
                        name.to_string()
                    }
                })
                .collect::<Vec<_>>();

            let rets = lua_func.get_ret();

            let ret_strs = rets
                .iter()
                .map(|ty| humanize_type(db, ty))
                .collect::<Vec<_>>()
                .join(",");

            let mut result = String::new();
            result.push_str(async_prev);
            result.push_str(local_prev);
            result.push_str("function ");
            result.push_str(func_name);
            result.push_str("(");
            if params.len() > 1 {
                result.push_str("\n");
                for param in &params {
                    result.push_str("  ");
                    result.push_str(param);
                    result.push_str(",\n");
                }
                result.pop(); // Remove the last comma
                result.pop(); // Remove the last newline
                result.push_str("\n");
            } else {
                result.push_str(&params.join(", "));
            }
            result.push_str(")");
            if ret_strs.len() > 15 {
                result.push_str("\n");
            }

            if !ret_strs.is_empty() {
                result.push_str("-> ");
                result.push_str(&ret_strs);
            }

            result
        }
        LuaType::Signature(signature_id) => {
            hover_signature_type(db, signature_id.clone(), func_name, is_local).unwrap_or(format!(
                "{}function {}",
                if is_local { "local " } else { "" },
                func_name
            ))
        }
        _ => format!(
            "{}function {}",
            if is_local { "local " } else { "" },
            func_name
        ),
    }
}

fn hover_signature_type(
    db: &DbIndex,
    signature_id: LuaSignatureId,
    func_name: &str,
    is_local: bool,
) -> Option<String> {
    let signature = db.get_signature_index().get(&signature_id)?;
    let mut async_prev = "";
    if let Some(property) = db
        .get_property_index()
        .get_property(LuaPropertyOwnerId::Signature(signature_id))
    {
        async_prev = if property.is_async { "async " } else { "" };
    }

    let local_prev = if is_local { "local " } else { "" };
    let params = signature
        .get_type_params()
        .iter()
        .map(|param| {
            let name = param.0.clone();
            if let Some(ty) = &param.1 {
                format!("{}: {}", name, humanize_type(db, ty))
            } else {
                name.to_string()
            }
        })
        .collect::<Vec<_>>();

    let rets = &signature.return_docs;

    let mut result = String::new();
    result.push_str(async_prev);
    result.push_str(local_prev);
    result.push_str("function ");
    result.push_str(func_name);
    result.push_str("(");
    if params.len() > 1 {
        result.push_str("\n");
        for param in &params {
            result.push_str("  ");
            result.push_str(param);
            result.push_str(",\n");
        }
        result.pop(); // Remove the last comma
        result.pop(); // Remove the last newline
        result.push_str("\n");
    } else {
        result.push_str(&params.join(", "));
    }
    result.push_str(")");
    match rets.len() {
        0 => {}
        1 => {
            result.push_str(" -> ");
            let type_text = humanize_type(db, &rets[0].type_ref);
            let name = rets[0].name.clone().unwrap_or("".to_string());
            let detail = if rets[0].description.is_some() {
                format!(" -- {}", rets[0].description.as_ref().unwrap())
            } else {
                "".to_string()
            };
            result.push_str(format!("{}{}{}", name, type_text, detail).as_str());
        }
        _ => {
            result.push_str("\n");
            for ret in rets {
                let type_text = humanize_type(db, &ret.type_ref);
                let name = ret.name.clone().unwrap_or("".to_string());
                let detail = if ret.description.is_some() {
                    format!(" -- {}", ret.description.as_ref().unwrap())
                } else {
                    "".to_string()
                };
                result.push_str(format!(" -> {}{}{}\n", name, type_text, detail).as_str());
            }
        }
    }

    Some(result)
}
