use serde_json::Value;

use crate::context::ServerContextSnapshot;

pub const COMMAND: &str = "emmy.fix.format";

#[allow(unused)]
pub async fn handle(context: ServerContextSnapshot, args: Vec<Value>) -> Option<()> {
    Some(())
}
