#[derive(Debug, serde::Deserialize)]
pub struct UserCreationRequest {
    pub username: String,
}

