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


use std::collections::HashMap;

use futures::FutureExt;
use iroh::{Endpoint, SecretKey, endpoint::{self, Connection, RecvStream, presets}};
use crate::internal::TaskWrapper;

const ALPN: &str = "evergreen/0.1.0";

pub enum ClientUpdate {
    /// Different from [ClientUpdate::NewPeer]. Emitted when a peer *attempts* to connect to this client
    IncomingPeer(iroh::EndpointId),
    /// Emitted when a peer has completed a full handshake and connected to this peer
    NewPeer(iroh::EndpointId),
}

/// A function that is called for updates on the client. 
/// 
/// This is an alternative to [Client::poll()]. It is recommended to __either__ use a callback __or__ to poll the client.
/// 
/// 
pub type UpdateCallback = fn(ClientUpdate) -> dyn Future<Output = ()>;

impl Client {

    /// Creates a [Client]. This starts a few things in motion at once.
    /// 
    /// 1. Creates an [Iroh][iroh] endpoint. This kickstarts discoverability via DNS and begins connections to relays
    /// 
    /// 2. Spins up a [Tokio task][tokio::task] for the main event loop.
    /// 
    /// Once this functions returns, the endpoint is "online" and has connected to a relay.
    /// 
    /// This function waits for [Endpoint::online()] to return. 
    /// Please refer to Iroh documentation regarding setting timeouts, as this function has none implemented by default.
    /// 
    /// This function takes an optional callback for updates about the client. 
    /// See [UpdateCallback] for more information.
    pub async fn new(identity: SecretKey, callback: Option<UpdateCallback>) -> Result<Self, iroh::endpoint::BindError> {

        let endpoint = Endpoint::builder(presets::N0)
            .alpns(vec![ALPN.into()])
            .secret_key(identity)
            .bind().await?;

        // Communication is key in any good relationship
        let (request_sender, request_receiver) = mpsc::channel::<Request>(128);
        let (response_sender, response_receiver) = mpsc::channel::<ClientUpdate>(128);

        // Safeword
        let token = tokio_util::sync::CancellationToken::new();

        // Riiise, RIIIIIIIIIIISE. GO MY MINIONS. LET TERROR REIGN.
        let task_handle = tokio::task::spawn(
            event_loop(endpoint.clone(), token.clone(), request_receiver, response_sender, callback)
        );

        let wrapper = TaskWrapper{
            cancel_token: token,
            task_handle, 
            incoming_queue: response_receiver, 
            outgoing_queue: request_sender
        };

        // Make sure we're actually online
        endpoint.online().await;

        Ok(Client{
            endpoint,
            task: wrapper,
        })
    }

    /// This functions takes ownership of a [Client] and then kills it. 
    /// This ends the internal event loop, closes all remote connections, and then stops the endpoint.
    pub async fn stop_client(self) {
        self.task.cancel();
    }
}


/// Wraps an iroh::Endpoint and a lot of other stuff in order to make networking and connections as easy as possible
/// 
/// Setting up networking can be as easy as:
/// ```
/// let secret = iroh::SecretKey::generate();
/// 
/// let client = firesidexr_evergreen::Client::new(secret, None).await;
/// ```
/// 
/// This client will then sit on the network waiting for incoming connections
/// 
/// 
pub struct Client {
    endpoint: Endpoint,
    task: TaskWrapper<(), ClientUpdate, Request>
}

use tokio::{pin, sync::mpsc};
use tokio_util::sync::CancellationToken;

enum Request{
    /// This adds a connection to the list of peers internally but does not make an actual connection on its own
    AddConnection(endpoint::Connection),

}

impl Client {
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        self.task.is_done()
    }

    #[inline(always)]
    pub async fn poll(&mut self) -> Option<ClientUpdate> {
        self.task.recv().await
    }

    /// Connects to a peer. 
    /// 
    /// This is a basic function that lacks normal group join features. 
    /// 
    /// # Panic
    /// This function panics if it is called on an invalid client. 
    /// If [Client::is_running()] returns true, you shouldn't have an issue. 
    /// 
    /// Honestly I'm not sure how the internal channels would become invalid but if it does then it's kinda a failure state.
    pub async fn connect_to_peer(&self, id: iroh::EndpointId) -> Result<(), endpoint::ConnectError> {
        let connection = self.endpoint.connect(id, ALPN.as_bytes()).await?;

        self.task.send(Request::AddConnection(connection)).await
        .expect("Internal request channel dropped or closed. Client is not longer valid");

        Ok(())
    }


    /// 
    /// 
    /// 
    pub async fn connect_to_room(&self) -> Result<(), ()> {


        

        todo!()
    }

}


// enum PeerState {

// }



mod connection_loop;

async fn event_loop(
    endpoint: Endpoint,
    cancel_token: tokio_util::sync::CancellationToken, 
    mut incoming_requests: mpsc::Receiver<Request>, 
    outgoing_updates: mpsc::Sender<ClientUpdate>,
    _callback: Option<UpdateCallback>,
){
    // // Probably overkill but ¯\_(ツ)_/¯
    // let mut read_buffer: Vec<u8> = Vec::with_capacity(1028);
    
    let mut room_id: Option<u64> = None;
    
    let mut connections = ConnectionList { list: vec![] };

    let mut incoming_connections = connection_loop::IncomingList { list: vec![]};

    let mut peer_info: HashMap<iroh::EndpointId, PeerInfo> = HashMap::with_capacity(128);

    loop {
        tokio::select! {
            // Shut it down!!!
            _ = cancel_token.cancelled() => {

                return;
            },
            // Requests *to* the client. Usually to 
            new_command = incoming_requests.recv() => { 
                let Some(command) = new_command else {break};

                match command {
                    Request::AddConnection(connection) => {
                        connections.list.push(connection)
                    },
                }

                //tokio::task::spawn(handle_request(endpoint.clone(), command, outgoing.clone()));
            },
            new_connection = endpoint.accept() => {
                match new_connection {
                    // Endpoint closed
                    None => cancel_token.cancel(),
                    // Someone is calling us :O
                    Some(incoming) => {
                        let handle = tokio::task::spawn(handle_incoming(endpoint.clone(), incoming));
                        incoming_connections.list.push(handle);
                    },
                }
            },
            (incoming_index, status) = &mut incoming_connections => {
                incoming_connections.list.remove(incoming_index);
                
                match status {
                    Err(error) => todo!(),
                    Ok(_) => {

                    },
                }
            }
            (connection_index, stream_event) = &mut connections => {
                match stream_event {
                    Err(_error) => { 
                        // All of the errors that connection.accept_uni() produces are failures of the connection
                        connections.list.remove(connection_index);
                    }
                    Ok(stream) => {
                        let _ = tokio::task::spawn(handle_packet(endpoint.clone(), stream, outgoing_updates.clone()));
                    },
                }
            }
        }
    }
}


async fn handle_packet(endpoint: Endpoint, mut stream: RecvStream, outoging: mpsc::Sender<ClientUpdate>) {

    let Ok(raw_packet) = stream.read_to_end(512).await else {return};

    let Ok(packet) = crate::wire::deserialize_packet(&raw_packet) else {return ;};

    todo!()
}

struct ConnectionList {
    list: Vec<Connection>,
}


impl Future for ConnectionList {
    type Output = (usize, Result<endpoint::RecvStream, endpoint::ConnectionError>);

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        
        for (index, connection) in self.list.iter().enumerate() {
            let future = connection.accept_uni();
            pin!(future);

            if let Poll::Ready(val) = future.poll(cx) {
                return Poll::Ready((index, val));
            };
        }
        
        Poll::Pending
    }
}

