/// Home for everything related to the lower-level functions of Evergreen. 
/// 
/// While the [client][crate::client] module defines a friendlier API for interfacing with Evergreen, 
/// this module acts more closely with Iroh.


/// GLORY TO THE ALPN
pub const ALPN: &str = "evergreen/0.1.0";



use tokio::sync::{broadcast, mpsc, watch};

// tokio's RwLock fairness is not desirable for our use case
// We want our blocklist to update instantly
use std::sync::{Arc, RwLock};

// I'd rather both of these types were wrapped into one type for convenience but honestly it's probably for the best.

mod filter;
mod handler;

pub use handler::EvergreenHandler;

/// Acts as your handle to interact with the connections routed to the [EvergreenHandler]
/// 
/// 
pub struct Evergreen
{
    connection_updates: mpsc::Receiver<()>,
    endpoint: iroh::Endpoint,

    // We don't actually use this as a handler internally. 
    // Inside of this struct we use it to reference and update data used by the actual handler (inside the router) 
    room_id: watch::Sender<Option<u64>>,
    broadcast: broadcast::Sender<()>,
    blocklist: Arc<RwLock<Vec<iroh::EndpointId>>>,

    
}


pub fn new(endpoint: iroh::Endpoint) -> (Evergreen, EvergreenHandler) {
    let room_id = watch::Sender::new(None);
        // gives us around a minute of packets to build up in memory. Probably too much but eh, I'll tank the higher memory usage.
        let broadcast = broadcast::Sender::<()>::new(1028);
        let (update_sender, connection_updates) = mpsc::channel::<()>(1028);

        let blocklist = Arc::new(RwLock::new(Vec::with_capacity(128)));

        let filter = Arc::new(filter::default_filter);

        

        let evergreen = Evergreen {
            connection_updates,
            endpoint,
            room_id: room_id.clone(), 
            broadcast: broadcast.clone(),
            blocklist: blocklist.clone(),
        };

        let handler = EvergreenHandler {
            room_id, 
            broadcast, 
            update_sender, 
            blocklist, 
            filter,
        };

        (evergreen, handler)
}


impl Evergreen {

    pub async fn join_room() {
        todo!()
    }



    pub async fn recv(&mut self) -> Option<()> {
        self.connection_updates.recv().await
    }
}