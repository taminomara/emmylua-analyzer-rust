#[derive(Debug, PartialEq)]
pub enum VisibilityKind {
    None,
    Public,
    Protected,
    Private,
    Internal,
    Package,
}

impl VisibilityKind {
    pub fn to_visibility_kind(visibility: &str) -> VisibilityKind {
        match visibility {
            "public" => VisibilityKind::Public,
            "protected" => VisibilityKind::Protected,
            "private" => VisibilityKind::Private,
            "internal" => VisibilityKind::Internal,
            "package" => VisibilityKind::Package,
            _ => VisibilityKind::None,
        }
    }
}
