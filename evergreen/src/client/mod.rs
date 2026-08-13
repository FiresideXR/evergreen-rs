
use iroh::{Endpoint, protocol::{RouterBuilder, Router}};

use tokio::sync::{broadcast, mpsc};
use tokio_util::sync::CancellationToken;

use std::{collections::HashMap, sync::Arc};
mod sync;

const ALPN: &'static str =  "evergreen/0.1.0";



pub enum RoomTransition {
    /// Disconnect from the current room, then connect to the next room. (Most standard model)
    LoadingScreen,
    /// Wait to disconnect from previous room until fully connected to the next room. Transition to next room can then be instant.
    Instant,
    /// Connect to multiple rooms at the same time. Broadcast your packets to all rooms at once. 
    Simultaneous,
}

pub struct Client {
    
    token: CancellationToken,

    task: tokio::task::JoinHandle<()>,

    //shared_state: Arc<sync::ClientSharedState>,

    send_commands: broadcast::Sender<()>,

    recv_updates: mpsc::Receiver<()>,

}

use crate::types::Identity;

impl Client {

    /// Creates a [Client]. This starts a few things in motion at once.
    /// 
    /// 
    pub async fn new(identity: Identity, transition: RoomTransition) -> Result<Self, iroh::endpoint::BindError> {

        let endpoint = Endpoint::builder(iroh::endpoint::presets::N0)
            .alpns(vec![ALPN.into()])
            .secret_key(identity)
            .bind().await?;

        // Make sure we're actually online
        endpoint.online().await;

        let token = CancellationToken::new();
        

        Ok(Client{

        })
    }
    /// This functions takes ownership of a [Client] and then kills it. 
    pub async fn stop_client(self) {
        let _ = self
    }
}



fn client_loop(token: CancellationToken) {

    let rooms: sync::RoomList = Arc::new(tokio::sync::RwLock::new(HashMap::with_capacity(8)));

    let connections: Vec<tokio::task::JoinHandle<()>> = Vec::with_capacity(32);

    let client_state: sync::ClientSharedState;




}
