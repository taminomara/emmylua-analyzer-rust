mod env_provider;
mod file_path_provider;
mod keywords_provider;
mod member_provider;
mod module_path_provider;
mod doc_tag_provider;

use super::completion_builder::CompletionBuilder;

pub fn add_completions(builder: &mut CompletionBuilder) -> Option<()> {
    module_path_provider::add_completion(builder);
    file_path_provider::add_completion(builder);
    keywords_provider::add_completion(builder);
    env_provider::add_completion(builder);
    member_provider::add_completion(builder);
    doc_tag_provider::add_completion(builder);

    Some(())
}
