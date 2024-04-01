use socketioxide::extract::{
    SocketRef,
    State,
    Data,
};
use tracing::{
    info,
    error,
};
use crate::{file_tracking::DirectoryTracker, user_management::{
    User, UserCreationRequest, UserStore
}};

/// Handle the join message sent by a client after its connection
/// used to create the user in the user store, and also send back the files 
/// hosted in the multiedit server
pub async fn handle_join_request(
    user_socket: SocketRef,
    Data(data): Data::<UserCreationRequest>,
    user_store: State<UserStore>,
    project_tracker: State<DirectoryTracker>,
) {

    info!("A user is trying to connect: 
           - Socket: {:?}
           - User Data: {:?}", user_socket, data);

    let user = User::create(user_socket.id, data);

    match user_store.add_user(user_socket.id, user).await {
        Ok(_) => {
            info!("User was successfully created")
        },
        Err(err) => {
            error!("There was an error when creating the user: {:?}", err);

            let _ = user_socket.emit("user-creation-error", err.to_string());
        },
    };

    send_files_to_client(user_socket, project_tracker).await;
}

async fn send_files_to_client(
    user_socket: SocketRef,
    State(project_tracker): State<DirectoryTracker>,
) {
    match user_socket.emit("filetree", project_tracker) {
        Ok(_) => info!("Project sucessfully sent to client"),
        Err(err) => error!("There was an error when sending the files to the client: {}", err),
    }
}
