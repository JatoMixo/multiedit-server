use socketioxide::extract::{
    SocketRef,
    State,
    Data,
};
use tracing::error;
use crate::file_tracking::{FileChange, ProjectTracker};

#[derive(Debug, serde::Deserialize, Clone)]
pub struct ChangeRequest {
    file: std::path::PathBuf,
    start_index: usize,
    end_index: usize,
    content: String,
}

impl Into<FileChange> for ChangeRequest {
    fn into(self) -> FileChange {
        FileChange::new(
            self.start_index,
            self.end_index,
            self.content
        )
    }
}

pub async fn handle_apply_change(
    user_socket: SocketRef,
    Data(change_request): Data<ChangeRequest>,
    project_tracker: State<ProjectTracker>,
) {
    match project_tracker.apply_change_to_file(&change_request.file, &change_request.clone().into()) {
        Ok(_) => {},
        Err(err) => {
            error!("There was an error when user {:?} applied a change: {:?}", user_socket.id, err);

            let _ = user_socket.emit("file-error", err);
        }
    }

    // TODO: Send the change to other clients
}
