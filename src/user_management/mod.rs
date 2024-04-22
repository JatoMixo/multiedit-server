//! Everything related to managing the clients connected to the MultiEdit server.
mod user;
pub use user::User;

mod user_store;
pub use user_store::UserStore;

pub mod user_error;

mod user_creation;
pub use user_creation::UserCreationRequest;
