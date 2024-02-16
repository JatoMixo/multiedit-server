use socketioxide::extract::SocketRef;
use crate::user_management::user_configuration::UserConfigurationRequest;

#[derive(Debug)]
pub struct User {
    username: Option<String>,
    cursor: Cursor,
}

impl User {
    pub fn empty() -> User {
        User {
            username: None,
            cursor: Cursor::new(),
        }
    }

    pub fn configure(&mut self, new_user_config: UserConfigurationRequest) {
        self.configure_username(new_user_config.new_username);
    }

    fn configure_username(&mut self, new_username: Option<String>) {
        match new_username {
            None => (),
            Some(final_username) => self.username = Some(final_username),
        }
    }
}

/// The position of a cursor in the screen, to show it in the rest of the clients
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Cursor {
    column: u32,
    row: u32,
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor {
            column: 0,
            row: 0,
        }
    }
}
