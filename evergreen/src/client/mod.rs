// Copyright (c) 2026 Anders Olsen
//
// Permission is hereby granted, free of charge, to any person obtaining 
// a copy of this software and associated documentation files (the "Software"), 
// to deal in the Software without restriction, including without limitation the 
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is 
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in 
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, 
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, 
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS 
// OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN 
// AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH 
// THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

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

impl Client {

    /// Creates a [`Client`] from a [`ClientConfig`](config::ClientConfig) and begins running it. 
    /// 
    /// 
    pub async fn create_and_start(config: config::ClientConfig) -> Result<Self, iroh::endpoint::BindError> {

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

    let (send_updates, recv_updates) = tokio::sync::mpsc::channel::<()>(1024);
    
    let state: sync::SharedState = Arc::new(sync::ClientState::new(Vec::new(), Vec::new(), send_updates));

    let task_handles: Vec<JoinHandle<()>> = Vec::with_capacity(32);


    loop {
        tokio::select! {
    
            _ = token.cancelled() => {
                for task in task_handles {
                    task.abort();
                }
                break;
            }
    
            maybe_incoming = endpoint.accept() => {
                let incoming = match maybe_incoming {
                    None => {return},
                    Some(incoming) => incoming,
                };
    
                let task = sync::spawn_connection_loop(incoming, state.clone());
                
            } 
    
    
    
    
        }
    }
}




async fn handle_maybe_incoming(maybe_incoming: Option<iroh::endpoint::Incoming>, blocklist: &tokio::sync::RwLock<Vec<iroh::EndpointId>>) -> iroh::endpoint::Incoming {
    todo!()
}