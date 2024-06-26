/// Track the changes applied to a file/directory hosted in the server and store them to make it
/// possible to return to an earlier state of the file/directory
mod file_tracker;
pub use file_tracker::{
    FileTracker,
    FileChange,
};

mod path;
pub use path::Path;

mod error;
pub use error::FileTrackingError;

mod project_tracker;
pub use project_tracker::ProjectTracker;

