
use super::filter::*;

use tokio::sync::{broadcast, mpsc, watch};

// tokio's RwLock fairness is not desirable for our use case
// We want our blocklist to update instantly
use std::{fmt::Debug, sync::{Arc, RwLock}};

/// This is the type to be passed to the router. 
/// It takes incoming [iroh::endpoint::Connection] and spawns a new task to handle streams for that connection. 
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
    /// :P
    pub(crate)  room_id: watch::Sender<u64>,
    pub(crate) broadcast: broadcast::Sender<()>,
    pub(crate) update_sender: mpsc::Sender<()>,
    pub(crate) blocklist: Arc<RwLock<Vec<iroh::EndpointId>>>,
    pub(crate) filter: ConnectionFilter,
}

impl EvergreenHandler {
    #[inline(always)]
    pub fn set_custom_filter(&mut self, filter: ConnectionFilter) {
        self.filter = filter
    }
}

// Basically the derived implementation sin the filter function (which doesn't support debug)
impl Debug for EvergreenHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EvergreenHandler")
            .field("room_id", &self.room_id)
            .field("broadcast", &self.broadcast)
            .field("update_sender", &self.update_sender)
            .field("blocklist", &self.blocklist)
            .finish()
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

    #[error("Connection filtered")]
    Filter,

    #[error("Packet was not of type Init")]
    NotInit,

    #[error("Main task panicked")]
    PoisonLock,
}

impl From<std::sync::PoisonError<std::sync::RwLockReadGuard<'_, Vec<iroh::EndpointId>>>> for Error {
    fn from(_value: std::sync::PoisonError<std::sync::RwLockReadGuard<'_, Vec<iroh::EndpointId>>>) -> Self {
        Error::PoisonLock
    }
}

// Connection close values
const OK: u32 = 0;
const ERR_MALFORMED_PACKET: u32 = 1;
const ERR_NOT_ALLOWED: u32 = 2;

use crate::wire;

impl EvergreenHandler {

    async fn incoming_handshake(&self, connection: &iroh::endpoint::Connection) -> Result<(), Error> {

        if self.blocklist.read()?.contains(&connection.remote_id()) {
            connection.close(ERR_NOT_ALLOWED.into(), "hit incoming filter".as_bytes());
            return Err(Error::Filter)
        }


        {
            let room_id = self.room_id.borrow();
        }

        let (send, mut recv) = connection.accept_bi().await?;

        let data = recv.read_to_end(512).await?;

        let packet = match wire::deserialize_packet(&data)? {
            wire::Packet::Init(initial_payload) => initial_payload,
            _ => {
                connection.close(ERR_MALFORMED_PACKET.into(), "Initial packet was not of correct type".as_bytes());
                return Err(Error::NotInit)
            },
        };

        if let FilterResult::CloseConnection = (self.filter)(&connection.remote_id(), &packet) {
            connection.close(ERR_NOT_ALLOWED.into(), "hit incoming filter".as_bytes());
            return Err(Error::Filter)
        };



        todo!()

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