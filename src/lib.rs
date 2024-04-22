//! The MultiEdit server is used to host a directory/file in a socketio server and let several
//! clients join, so that people can edit the same file from different computers across the
//! network, intended for pair-programming purposes.
pub mod user_management;
pub mod file_tracking;

mod server;
pub use server::start_server;

