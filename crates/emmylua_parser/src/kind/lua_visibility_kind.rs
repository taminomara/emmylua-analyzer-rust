#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum VisibilityKind {
    None,
    Public,
    Protected,
    Private,
    Internal,
    Package,
}

impl VisibilityKind {
    #[allow(unused)]
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

    #[allow(unused)]
    pub fn to_str(&self) -> Option<&'static str> {
        match self {
            VisibilityKind::None => None,
            VisibilityKind::Public => Some("public"),
            VisibilityKind::Protected => Some("protected"),
            VisibilityKind::Private => Some("private"),
            VisibilityKind::Internal => Some("internal"),
            VisibilityKind::Package => Some("package"),
        }
    }
}
