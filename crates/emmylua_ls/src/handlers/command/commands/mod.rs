use serde_json::Value;

use crate::context::ServerContextSnapshot;

mod emmy_auto_require;
mod emmy_disable_code;
mod emmy_fix_format;

pub use emmy_disable_code::{make_disable_code_command, DisableAction};

pub fn get_commands_list() -> Vec<String> {
    let mut commands = Vec::new();
    macro_rules! command_from {
        ($($module:ident),*) => {
            $(
                let command_str = $module::COMMAND.to_string();
                commands.push(command_str);
            )*
        };
        () => {

        };
    }

    command_from!(emmy_auto_require);
    command_from!(emmy_disable_code);
    command_from!(emmy_fix_format);

    commands
}

pub async fn dispatch_command(
    context: ServerContextSnapshot,
    command_name: &str,
    args: Vec<Value>,
) -> Option<()> {
    Some(())
}
