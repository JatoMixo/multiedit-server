use crate::User;
use crate::user_management::user_error::UserCreationError;
use std::collections::HashMap;
use socketioxide::socket::Sid;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct UserStore {
    pub users: RwLock<HashMap<Sid, User>>,
}

impl UserStore {
    pub async fn add_user(&self, user_id: Sid, user_data: User) -> Result<(), UserCreationError> {
        if self.is_id_taken(user_id).await {
            return Err(UserCreationError::UserAlreadyExists);
        }

        let mut users_binding = self.users.write().await;

        users_binding.insert(user_id, user_data);

        Ok(())
    } 

    async fn is_id_taken(&self, id: Sid) -> bool {
        self
            .users
            .read()
            .await
            .contains_key(&id)
    }
}
