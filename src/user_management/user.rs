use crate::user_management::UserCreationRequest;
use socketioxide::socket::Sid;

/// A user connected to the server
#[derive(Debug, Clone)]
pub struct User {
    /// The id of the user
    /// TODO: Change from a socket id to a custom id type
    id: Sid,
    /// The username chosen by the user
    username: String,
}

impl User {
    /// Crate a new user with its connection id and with the data from the join request
    pub fn create(id: Sid, user_creation_data: UserCreationRequest) -> User {
        User {
            id,
            username: user_creation_data.username,
        }
    }
}
