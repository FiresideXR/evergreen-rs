
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex, broadcast, mpsc, watch};
use tokio_util::sync::CancellationToken;

use std::collections::HashMap;



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
// Perf gain would also probably be negligible. This data isn't being allocated and deallocated frequently.
pub type SharedState = Arc<ClientState>;

pub enum Events {

}

pub enum InternalEvents {
    GatherPeers(RoomID, mpsc::Sender<String>),
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

    /// Rooms this peer is _active_ in.
    /// 
    /// Active means that this client is ready to send and receive data within that room.
    /// If this peer and a remote peer share rooms, they will exchange data.
    /// 
    /// Peers will clean up connections that don't share active rooms.
    pub rooms: watch::Sender<Vec<RoomID>>,


    /// 
    pub peers: RwLock<HashMap<RoomID, Vec<String>>>

}

impl ClientState {
    pub fn new(passports: Vec<crate::types::Passport>, blocklist: Vec<iroh::EndpointId>) -> ClientState {
        Self {
            passports: RwLock::new(passports),
            blocklist: RwLock::new(blocklist),
            recv_packets: broadcast::Sender::new(1024),
            rooms: watch::Sender::new(Vec::with_capacity(8)),
            peers: RwLock::new(HashMap::with_capacity(8)),
        }
    }

}



pub async fn connection_loop(state: SharedState, event_sender: mpsc::Sender<Events>) {

    




}