use std::path::Path;

use code_analysis::{humanize_type, DbIndex, LuaMemberOwner, LuaPropertyOwnerId, LuaTypeDecl};
use serde::{Deserialize, Serialize};
use tera::Tera;

pub fn generate_type_markdown(
    db: &DbIndex,
    tl: &Tera,
    typ: &LuaTypeDecl,
    output: &Path,
) -> Option<()> {
    let mut context = tera::Context::new();
    let typ_name = typ.get_name();
    context.insert("type_name", &typ_name);

    let typ_id = typ.get_id();
    let namespace = typ.get_namespace();
    context.insert("namespace", &namespace);

    let type_property_id = LuaPropertyOwnerId::TypeDecl(typ_id.clone());
    let typ_property = db.get_property_index().get_property(type_property_id);
    if let Some(typ_property) = typ_property {
        if let Some(property_text) = &typ_property.description {
            context.insert("description", &property_text);
        }
    }

    let supers = db.get_type_index().get_super_types(&typ_id);
    if let Some(supers) = supers {
        let mut super_type_texts = Vec::new();
        for super_typ in supers {
            let super_type_text = humanize_type(db, &super_typ);
            super_type_texts.push(super_type_text);
        }
        context.insert("super_types", &super_type_texts.join(", "));
    }

    let member_owner = LuaMemberOwner::Type(typ_id);
    let member_map = db.get_member_index().get_member_map(member_owner);
    let mut method_members: Vec<MethodMember> = Vec::new();
    let mut field_members: Vec<FieldMember> = Vec::new();
    if let Some(member_map) = member_map {
        for (member_name, member_id) in member_map {
            let member = db.get_member_index().get_member(member_id)?;
            let member_typ = member.get_decl_type();
        }
    }
    context.insert("methods", &method_members);
    context.insert("fields", &field_members);
    // let markdown

    let render_text = tl.render("lua_type_template.tl", &context).ok()?;
    let outpath = output.join(format!("{}.md", typ.get_full_name()));
    std::fs::write(outpath, render_text).ok()?;
    Some(())
}

#[derive(Debug, Serialize, Deserialize)]
struct MethodMember {
    pub display_name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FieldMember {
    pub display_name: String,
    pub description: String,
}
