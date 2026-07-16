/// Home for everything related to the lower-level functions of Evergreen. 
/// 
/// While the [client][crate::client] module defines a friendlier API for interfacing with Evergreen, 
/// this module acts more closely with Iroh.



use tokio::sync::{broadcast, mpsc, watch};

// tokio's RwLock fairness is not desirable for our use case
// We want our blocklist to update instantly
use std::{fmt::Debug, sync::{Arc, RwLock}};

/// GLORY TO THE ALPN
pub const ALPN: &str = "evergreen/0.1.0";


pub enum FilterResult {
    Close,
}

// You have no idea how much time I spent trying to make this type any cleaner than iroh::protocol::IncomingFilter and just totally failed
// 
// In an ideal world I'd have this optimized into the handler itself but at this point I truly have stopped caring
// Who care about v-tables. This function gets called once per connection. Literally doesn't matter.
/// I hate this type so much more than you think I possibly could
pub type ConnectionFilter = Arc<dyn Fn(&iroh::EndpointId, &wire::InitialPayload) -> FilterResult + Send + Sync>;

fn default_filter(addr: &iroh::EndpointId, info: &wire::InitialPayload) -> FilterResult {

    todo!()
}

/// This is the type passed to the router via [Evergreen::handle()]
/// 
/// 
#[derive(Clone)]
pub struct EvergreenHandler
{
    /// Where we are
    /// 
    /// We need to listen for updates to this value across tasks and perform operations based on it
    /// We also need to compare the current state of this value 
    /// 
    /// We could choose to include this in our broadcast enum but I want to keep that logic cleaner.
    /// This could also be a Arc Mutex but then we wouldn't get notified of changes and would need another handle for that.
    room_id: watch::Sender<u64>,
    broadcast: broadcast::Sender<()>,
    update_sender: mpsc::Sender<()>,
    blocklist: Arc<RwLock<Vec<()>>>,
    filter: ConnectionFilter,
}

// Basically the derived implementation sin the filter function (which doesn't support debug)
impl Debug for EvergreenHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EvergreenHandler").field("room_id", &self.room_id).field("broadcast", &self.broadcast).field("update_sender", &self.update_sender).field("blocklist", &self.blocklist).finish()
    }
}

// I'd rather both of these types were wrapped into one type for convenience but honestly it's probably for the best.
pub struct Evergreen
{
    connection_updates: mpsc::Receiver<()>,
    endpoint: iroh::Endpoint,

    // We don't actually use this as a handler internally. 
    // Inside of this struct we use it to reference and update data used by the actual handler (inside the router) 
    room_id: watch::Sender<u64>,
    broadcast: broadcast::Sender<()>,
    blocklist: Arc<RwLock<Vec<()>>>,
}


pub fn new(endpoint: iroh::Endpoint) -> (Evergreen, EvergreenHandler) {
    let room_id = watch::Sender::new(0);
        // gives us around a minute of packets to build up in memory. Probably too much but eh, I'll tank the higher memory usage.
        let broadcast = broadcast::Sender::<()>::new(1028);
        let (update_sender, connection_updates) = mpsc::channel::<()>(1028);

        let blocklist = Arc::new(RwLock::new(Vec::with_capacity(128)));

        let filter = Arc::new(default_filter);

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

// thiserror my beloved <3. Oh how you have changed my life
#[derive(thiserror::Error, Debug)]
enum Error {

    #[error(transparent)]
    ConnectionError(#[from] iroh::endpoint::ConnectionError),

    #[error("Packet provided was too large")]
    PacketSizeError(#[from] iroh::endpoint::ReadToEndError),

    #[error("Could not deserialize packet")]
    ProtoBufError(#[from] protobuf::ParseError),

    #[error("Packet was not of type Init")]
    NotInit,
}

// Connection close values
const OK: u32 = 0;
const ERR_MALFORMED_PACKET: u32 = 1;
const ERR_NOT_ALLOWED: u32 = 2;

use iroh::protocol::AcceptError;

use crate::wire;

impl EvergreenHandler {

    async fn incoming_handshake(&self, connection: &iroh::endpoint::Connection) -> Result<(), Error> {

        {
            let room_id = self.room_id.borrow();
        }

        let (send, mut recv) = connection.accept_bi().await?;

        let data = recv.read_to_end(512).await?;

        let packet = wire::deserialize_packet(&data)?;

        let wire::Packet::Init(payload) = packet else {return Err(Error::NotInit)};

        

        let filter_result = (self.filter)(&connection.remote_id(), &payload);

        todo!();
    }

    async fn outgoing_handshake(&self, connection: &iroh::endpoint::Connection) -> Result<(), Error> {


        todo!()
    }

    async fn event_loop(&self, connection: iroh::endpoint::Connection) -> Result<(), Error> {

        let room = self.room_id.subscribe();
        let packets = self.broadcast.subscribe();


        todo!()
    }
}

// Why does ProtocolHandler need to impl Debug. Who needs that. What's the use.
impl iroh::protocol::ProtocolHandler for EvergreenHandler {
    async fn accept(&self, connection: iroh::endpoint::Connection) -> Result<(), iroh::protocol::AcceptError> {

        // I shouldn't have to map this error myself. This should be a From<> implementation. 
        // Insane to me. Afaik this is like three lines of code to fix on iroh's end.
        // 
        // Also BLOWS my mind that I can't write an Into<> impl that gets used by ?
        // Like why is only From<> supported. Make that make sense to me.
        // 
        // Either way this should just be a .await? instead of this whole map that is still going!!
        self.incoming_handshake(&connection).await.map_err(iroh::protocol::AcceptError::from_err)?;

        self.event_loop(connection).await.map_err(iroh::protocol::AcceptError::from_err)
    }
}