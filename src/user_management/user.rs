#[derive(Debug, Clone)]
pub struct User {
    username: String,
    cursor: Cursor,
}

impl User {
    pub fn create(user_creation_data: UserCreationRequest) -> User {
        User {
            username: user_creation_data.username,
            cursor: Cursor::new(),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct UserCreationRequest {
    pub username: String,
}

/// The position of a cursor in the screen, to show it in the rest of the clients
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
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
