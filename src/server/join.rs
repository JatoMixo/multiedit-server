use socketioxide::{extract::{
    Data, SocketRef, State
}, socket::Sid};
use tracing::{
    info,
    error,
};
use crate::{file_tracking::ProjectTracker, user_management::{
    User, UserCreationRequest, UserStore
}};

#[derive(serde::Serialize)]
struct ClientConnectionResponse {
    user_id: String,
    username: String,
}

impl ClientConnectionResponse {
    pub fn new(user_id: Sid, username: String) -> ClientConnectionResponse {
        ClientConnectionResponse {
            user_id: user_id.to_string(),
            username,
        }
    }
}

/// Handle the join message sent by a client after its connection
/// used to create the user in the user store, and also send back the files 
/// hosted in the multiedit server
pub async fn handle_join_request(
    user_socket: SocketRef,
    Data(data): Data::<UserCreationRequest>,
    user_store: State<UserStore>,
    project_tracker: State<ProjectTracker>,
) {

    info!("A user is trying to connect: 
           - Socket: {:?}
           - User Data: {:?}", user_socket, data);

    let user = User::create(user_socket.id, &data);

    match user_store.add_user(user_socket.id, user).await {
        Ok(_) => {
            info!("User was successfully created")
        },
        Err(err) => {
            error!("There was an error when creating the user: {:?}", err);

            let _ = user_socket.emit("user-creation-error", err.to_string());
        },
    };

    let _ = user_socket.broadcast().emit("client-connected", ClientConnectionResponse::new(user_socket.id, data.username));

    send_files_to_client(user_socket, project_tracker).await;
}

async fn send_files_to_client(
    user_socket: SocketRef,
    State(project_tracker): State<ProjectTracker>,
) {
    match project_tracker.get_file_content_tree() {
        Err(err) => {
            error!("Error getting file content tree: {:?}", err);

            let _ = user_socket.emit("file-content-error", err);
            let _ = user_socket.disconnect();
        },
        Ok(file_content_tree) => {
            if let Err(err) = user_socket.emit("file-content-tree", file_content_tree) {
                error!("Error sending file content tree to client: {:?}", err);
            }
        }
    }
}
