
use iroh::{Endpoint, protocol::{RouterBuilder, Router}};

use tokio::{sync::{broadcast, mpsc}, task::JoinHandle};
use tokio_util::sync::CancellationToken;

use std::{collections::HashMap, sync::Arc};
mod sync;
pub mod config;

const ALPN: &'static str =  "evergreen/0.1.0";





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
    pub async fn new(config: config::ClientConfig) -> Result<Self, iroh::endpoint::BindError> {

        let endpoint = Endpoint::builder(iroh::endpoint::presets::N0)
            .alpns(vec![ALPN.into()])
            .secret_key(config.identity)
            .bind().await?;

        // Make sure we're actually online
        endpoint.online().await;

        let token = CancellationToken::new();
        let loop_token = token.clone();

        let (broadcast_send, task_recv) = broadcast::channel(256);
        let (task_send, handle_recv) = mpsc::channel::<()>(1024);

        let handle = tokio::spawn(async move  {
            client_loop(endpoint, loop_token, task_recv, task_send).await
        });
        
        Ok(Client{
            token,
            task: handle,
            send_commands: broadcast_send,
            recv_updates: handle_recv,
        })
    }
    /// This functions takes ownership of a [Client] and then kills it. 
    pub async fn stop_client(self) {
        let _ = self;
    }
}



async fn client_loop(endpoint: iroh::Endpoint, token: CancellationToken, broadcast_recv: broadcast::Receiver<()>, send_updates: mpsc::Sender<()>) {

    let state: sync::SharedState = Arc::new(sync::ClientState::new(Vec::new(), Vec::new()));

    let task_handles: Vec<JoinHandle<()>> = Vec::with_capacity(32);

    tokio::select! {

        _ = token.cancelled() => {
            for task in task_handles {
                task.abort();
            }
        }


        maybe_incoming = endpoint.accept() => {
            let incoming = match maybe_incoming {
                None => {return},
                Some(incoming) => incoming,
            };

            let task = sync::spawn_connection_loop(incoming, state.clone());
            
        } 




    }


    let Ok(conn) = incoming.await else {continue};

    screen

    conn.accept_bi().await


    todo!();
}




async fn handle_maybe_incoming(maybe_incoming: Option<Incoming>, blocklist: &RwLock<Vec<iroh::EndpointId>>) -> Incoming {
    todo!()
}