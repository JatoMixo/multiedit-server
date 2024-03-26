use socketioxide::{
    socket::DisconnectReason,
    extract::{
        SocketRef,
        State,
    }
};
use crate::{
    server::join::handle_join_request,
    user_management::user_store::UserStore,
};
use tracing::info;

pub async fn on_connect(socket: SocketRef) {
    info!("Socket connected: {}", socket.id);

    socket.on("join", handle_join_request);

    socket.on_disconnect(handle_socket_disconnection);
}

async fn handle_socket_disconnection(socket: SocketRef, disconnect_reason: DisconnectReason, user_store: State<UserStore>) {
    info!("Socket disconnected: {:?} -> {:?}", socket.id, disconnect_reason);
    user_store.remove_user_by_id(socket.id).await;
}
