mod instantiate_func_generic;
mod instantiate_special_generic;
mod instantiate_type_generic;
mod test;
mod tpl_context;
mod tpl_pattern;
mod type_substitutor;

pub use instantiate_func_generic::instantiate_func_generic;
pub use instantiate_type_generic::instantiate_doc_function;
pub use instantiate_type_generic::instantiate_type_generic;
pub use tpl_context::TplContext;
pub use tpl_pattern::tpl_pattern_match_args;
pub use type_substitutor::TypeSubstitutor;
