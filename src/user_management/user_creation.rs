/// The user creation request sent by the clients in the "join" message
/// containing the user's config and data such as the username
#[derive(Debug, serde::Deserialize)]
pub struct UserCreationRequest {
    pub username: String,
}
