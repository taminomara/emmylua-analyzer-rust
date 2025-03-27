#[derive(Debug)]
pub enum TypeCheckFailReason {
    DonotCheck,
    TypeNotMatch,
    TypeRecursion,
    TypeNotMatchWithReason(String),
}
