use crate::User;
use crate::user_management::user_error::UserCreationError;
use std::collections::HashMap;
use socketioxide::socket::Sid;

pub type UserId = Sid;

#[derive(Default)]
pub struct UserStore {
    pub users: HashMap<Sid, User>,
}

impl UserStore {
    pub fn add_user(&mut self, user_id: UserId, user_data: User) -> Result<(), UserCreationError> {
        if self.is_id_taken(user_id) {
            return Err(UserCreationError::UserAlreadyExists);
        }

        self
            .users
            .insert(user_id, user_data);

        Ok(())
    } 

    fn is_id_taken(&self, id: UserId) -> bool {
        self
            .users
            .contains_key(&id)
    }
}
