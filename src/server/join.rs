use socketioxide::extract::{
    SocketRef,
    State,
    Data,
};
use tracing::info;
use crate::user_management::{
    UserStore,
    User,
    UserCreationRequest,
};

/// Handle the join message sent by a client after its connection
/// used to create the user in the user store, and also send back the files 
/// hosted in the multiedit server
pub async fn handle_join_request(user_socket: SocketRef, Data(data): Data::<UserCreationRequest>, user_store: State<UserStore>) {

    info!("A user is trying to connect: 
           - Socket: {:?}
           - User Data: {:?}", user_socket, data);

    let user = User::create(user_socket.id, data);

    match user_store.add_user(user_socket.id, user).await {
        Ok(_) => {
            info!("User was successfully created")
        },
        Err(err) => {
            info!("There was an error when creating the user: {:?}", err);

            let _ = user_socket.emit("user-creation-error", err.to_string());
        },
    };

    // TODO: Send the files back to the client
}
