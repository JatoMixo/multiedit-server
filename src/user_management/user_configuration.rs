#[derive(Debug, serde::Deserialize)]
pub struct UserConfigurationRequest {
    pub new_username: Option<String>,
}

