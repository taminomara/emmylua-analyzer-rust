mod keywords_provider;
mod local_env_provider;

use super::completion_builder::CompletionBuilder;


pub fn add_completions(builder: &mut CompletionBuilder) -> Option<()> {
    keywords_provider::add_completion(builder);
    local_env_provider::add_completion(builder);
    Some(())
}