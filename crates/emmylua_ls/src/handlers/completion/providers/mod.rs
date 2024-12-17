mod keywords_provider;
mod env_provider;

use super::completion_builder::CompletionBuilder;


pub fn add_completions(builder: &mut CompletionBuilder) -> Option<()> {
    keywords_provider::add_completion(builder);
    env_provider::add_completion(builder);
    Some(())
}