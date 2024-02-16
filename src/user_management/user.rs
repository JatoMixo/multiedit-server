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

    pub fn create(&mut self, user_creation_data: UserCreationRequest) {
        self.username = Some(user_creation_data.username);
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct UserCreationRequest {
    pub username: String,
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
