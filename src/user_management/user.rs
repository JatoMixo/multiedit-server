use crate::Cursor;
use crate::UserCreationRequest;

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


