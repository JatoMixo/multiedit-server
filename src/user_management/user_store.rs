use crate::User;
use crate::user_management::user_error::UserCreationError;
use std::collections::HashMap;
use socketioxide::socket::Sid;
use tokio::sync::RwLock;

/// The store containing all the users, it's a hashmap where the id of the user
/// points to its actual data (the User struct)
#[derive(Default)]
pub struct UserStore {
    pub users: RwLock<HashMap<Sid, User>>,
}

impl UserStore {
    /// Add a new user to the store
    pub async fn add_user(&self, user_id: Sid, user_data: User) -> Result<(), UserCreationError> {
        if self.is_id_taken(user_id).await {
            return Err(UserCreationError::UserAlreadyExists);
        }

        let mut users_binding = self.users.write().await;

        users_binding.insert(user_id, user_data);

        Ok(())
    } 

    /// Check if a certain user id is already taken
    async fn is_id_taken(&self, id: Sid) -> bool {
        self
            .users
            .read()
            .await
            .contains_key(&id)
    }

    /// Remove a user from the store using its id
    pub async fn remove_user_by_id(&self, id: Sid) {
        self
            .users
            .write()
            .await
            .remove(&id);
    }
}
