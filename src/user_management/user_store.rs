use crate::User;
use crate::user_management::user_error::UserCreationError;

pub struct UserStore {
    pub users: Vec<User>,
}

impl UserStore {
    pub fn add_user(&mut self, user_to_add: User) -> Result<(), UserCreationError> {
        if self.is_id_taken(user_to_add.get_id()) {
            return Err(UserCreationError::UserAlreadyExists);
        }

        self.users.push(user_to_add);

        Ok(())
    } 

    fn is_id_taken(&self, id: &str) -> bool {
        self
            .users
            .iter()
            .map(|user| user.get_id())
            .collect::<Vec<&str>>()
            .contains(&id)
    }
}
