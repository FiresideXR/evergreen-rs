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


use std::sync::Arc;
use tokio::sync::{RwLock, broadcast, mpsc, watch};

use std::collections::HashMap;


#[derive(Debug, Clone, Copy)]
pub struct RoomID {
    id: [u8; 32],
}

impl From<u8> for RoomID {
    // Just a handy constructor for debugging
    fn from(value: u8) -> Self {
        Self {
            id: [value; 32]
        }
    }
}

// This could eventually be a Weak instead of Arc. Would require some unsafe code + more checks. 
// Perf gain would also probably be negligible. This Arc isn't being cloned and dropped *that* often.
pub type SharedState = Arc<ClientState>;

#[derive(Debug, Clone)]
pub enum NetworkEvent {
    // There are better ways to do this. I'd love to figure out some sort of shared arena. For now it's #goodenough
    /// Broadcast the data to all peers in shared rooms.
    SendData(Arc<[u8]>), 
    ChangeRooms(RoomID),
}

pub enum InternalEvents {
    GatherPeers(RoomID, mpsc::Sender<String>),
}

pub struct Update {
    pub sender: iroh::EndpointId,
    pub event: NetworkEvent,
}


/// State shared by every connection task
pub struct ClientState {
    // separate RwLocks to prevent potential slowdowns where writing to one field prevents accessing another 
    // 
    // makes the code more annoying to read and write but *should* prevent potential slowdowns

    
    /// Identities held by the client. Shared with other endpoints upon connection
    pub passports: RwLock<Vec<crate::types::Passport>>,
    
    /// People we hate (connections from this endpoint will be refused)
    pub blocklist: RwLock<Vec<iroh::EndpointId>>,

    /// Packets to be broadcast across the network
    pub recv_packets: broadcast::Sender<Vec<u8>>,

    pub send_updates: mpsc::Sender<()>,

    /// Rooms this peer is _active_ in.
    /// 
    /// Active means that this client is ready to send and receive data within that room.
    /// If this peer and a remote peer share rooms, they will exchange data.
    /// 
    /// Peers will clean up connections that don't share active rooms.
    pub rooms: watch::Sender<Vec<RoomID>>,


    /// 
    pub peers: RwLock<HashMap<RoomID, Vec<String>>>,

    

}

impl ClientState {
    pub fn new(passports: Vec<crate::types::Passport>, blocklist: Vec<iroh::EndpointId>, send_updates: mpsc::Sender<()>) -> ClientState {
        Self {
            passports: RwLock::new(passports),
            blocklist: RwLock::new(blocklist),
            recv_packets: broadcast::Sender::new(1024),
            send_updates,
            rooms: watch::Sender::new(Vec::with_capacity(8)),
            peers: RwLock::new(HashMap::with_capacity(8)),
        }
    }

}

/// Helper method for spawning a [`connection_loop`](connection_loop) in a task
pub async fn spawn_connection_loop(incoming: iroh::endpoint::Incoming, state: SharedState) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async {connection_loop(incoming, state)}.await)
}


pub async fn connection_loop(_incoming: iroh::endpoint::Incoming, _state: SharedState) {




    todo!()


}