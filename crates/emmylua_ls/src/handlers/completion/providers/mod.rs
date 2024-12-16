mod keywords_provider;

use super::completion_context::CompletionContext;


pub fn add_completions(context: &mut CompletionContext) -> Option<()> {
    keywords_provider::add_completion(context);
    Some(())
}