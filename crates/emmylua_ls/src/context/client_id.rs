use lsp_types::ClientInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClientId {
    VSCode,
    Intellij,
    Neovim,
    Other,
}

impl Default for ClientId {
    fn default() -> Self {
        ClientId::Other
    }
}

#[allow(unused)]
impl ClientId {
    pub fn is_vscode(&self) -> bool {
        matches!(self, ClientId::VSCode)
    }

    pub fn is_intellij(&self) -> bool {
        matches!(self, ClientId::Intellij)
    }

    pub fn is_neovim(&self) -> bool {
        matches!(self, ClientId::Neovim)
    }

    pub fn is_other(&self) -> bool {
        matches!(self, ClientId::Other)
    }
}

pub fn get_client_id(client_info: &Option<ClientInfo>) -> ClientId {
    match client_info {
        Some(info) => {
            if info.name == "Visual Studio Code" {
                ClientId::VSCode
            } else if info.name == "IntelliJ" {
                ClientId::Intellij
            } else if info.name == "Neovim" {
                ClientId::Neovim
            } else {
                ClientId::Other
            }
        }
        None => ClientId::Other,
    }
}
