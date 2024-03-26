use socketioxide::extract::{
    SocketRef,
    State,
    Data,
};
use tracing::info;
use crate::user_management::{
    user_store::UserStore,
    user::User,
    user_creation::UserCreationRequest,
};

pub async fn handle_join_request(user_socket: SocketRef, Data(data): Data::<UserCreationRequest>, user_store: State<UserStore>) {

    info!("A user is trying to connect: 
           - Socket: {:?}
           - User Data: {:?}", user_socket, data);

    let user = User::create(data);

    match user_store.add_user(user_socket.id, user).await {
        Ok(_) => {
            info!("User was successfully created")
        },
        Err(err) => {
            info!("There was an error when creating the user: {:?}", err);

            let _ = user_socket.emit("user-creation-error", err.to_string());
        },
    };
}
