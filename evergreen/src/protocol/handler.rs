
use super::filter::*;

use iroh_tickets::Ticket;
use tokio::sync::{broadcast, mpsc, watch};

// tokio's RwLock fairness is not desirable for our use case
// We want our blocklist to update instantly
use std::{fmt::Debug, str::FromStr, sync::{Arc, RwLock}};

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
    pub(crate) endpoint: iroh::Endpoint,
    
    pub(crate) room_id: watch::Sender<Option<u64>>,
    pub(crate) broadcast: broadcast::Sender<()>,
    pub(crate) update_sender: mpsc::Sender<()>,

    pub(crate) passports: Arc<RwLock<Vec<crate::types::Passport>>>,
    pub(crate) blocklist: Arc<RwLock<Vec<iroh::EndpointId>>>,
    pub(crate) peer_tickets: Arc<RwLock<Vec<String>>>,

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
    ProtoBufDeserialize(#[from] protobuf::ParseError),

    #[error("Could not serialize packet")]
    ProtoBufSerialize(#[from] protobuf::SerializeError),

    #[error("Connection filtered")]
    Filter,

    #[error("Packet was not of type Init")]
    NotInit,

    #[error("Main task panicked")]
    PoisonLock,

    #[error(transparent)]
    WriteError(#[from] iroh::endpoint::WriteError),

    #[error("Invalid remote ticket")]
    TicketParseError(#[from] iroh_tickets::ParseError),
}

impl <T> From<std::sync::PoisonError<T>> for Error {
    fn from(_value: std::sync::PoisonError<T>) -> Self {
        Error::PoisonLock
    }
}

// Connection close values
const OK: u32 = 0;
const ERR_MALFORMED_PACKET: u32 = 1;
const ERR_NOT_ALLOWED: u32 = 2;

use crate::wire;

impl EvergreenHandler {
    async fn incoming_handshake(&self, connection: &iroh::endpoint::Connection) -> Result<wire::InitialPayload, Error> {

        if self.blocklist.read()?.contains(&connection.remote_id()) {
            connection.close(ERR_NOT_ALLOWED.into(), "hit incoming filter".as_bytes());
            return Err(Error::Filter)
        }

        let (mut send, mut recv) = connection.accept_bi().await?;

        let data = recv.read_to_end(512).await?;

        let packet = match wire::deserialize_packet(&data)? {
            wire::Packet::Init(initial_payload) => initial_payload,
            _ => {
                connection.close(ERR_MALFORMED_PACKET.into(), "Initial packet was not of correct type".as_bytes());
                return Err(Error::NotInit)
            },
        };

        let ticket = iroh_tickets::endpoint::EndpointTicket::decode_string(&packet.ticket)?;

        // put this in default_filter?
        if ticket.endpoint_addr().id != connection.remote_id() {
            connection.close(ERR_MALFORMED_PACKET.into(), "Invalid ticket provided".as_bytes());
        }

        if let FilterResult::CloseConnection = (self.filter)(&connection.remote_id(), &packet) {
            connection.close(ERR_NOT_ALLOWED.into(), "hit incoming filter".as_bytes());
            return Err(Error::Filter)
        };

        let outgoing = wire::serialize_initial_payload(
            &iroh_tickets::endpoint::EndpointTicket::new(self.endpoint.addr()).to_string(),
            &self.passports.read()?,
            &self.peer_tickets.read()?,
            *self.room_id.borrow(),
            &[]
        )?;

        

        self.peer_tickets.write()?.push(ticket.to_string());

        send.write_all(&outgoing).await?;

        // Who even cares about error handling 
        // If this fails its just the other peer's fault and problem. rip bozo
        let _ = send.finish();

        Ok(packet)
    }



// IMPORTANT NOTE:
// 
// recv.read_to_end() calls read_chunk() repeatedly until it returns None.
// This means the read to end function will await until the remote peer calls finish()



    async fn outgoing_handshake(&self, connection: &iroh::endpoint::Connection) -> Result<(), Error> {

        let outgoing = wire::serialize_initial_payload(
            &iroh_tickets::endpoint::EndpointTicket::new(self.endpoint.addr()).to_string(),
            &self.passports.read()?,
            &self.peer_tickets.read()?,
            *self.room_id.borrow(), 
            &[]
        )?;

        let (mut send, mut recv) = connection.open_bi().await?;

        send.write_all(&outgoing).await?;

        // We have to first let the remote peer know that we are finished sending data
        // Second we wait until the peer has downloaded all of our data
        let _ = send.finish();
        let _ = send.stopped().await;

        let incoming = recv.read_to_end(1024).await?;

        let packet = match wire::deserialize_packet(&incoming)? {
            wire::Packet::Init(initial_payload) => initial_payload,
            _ => {
                connection.close(ERR_MALFORMED_PACKET.into(), "Initial packet was not of correct type".as_bytes());
                return Err(Error::NotInit)
            },
        };

        let ticket = iroh_tickets::endpoint::EndpointTicket::from_str(packet.ticket)?;

        if ticket.endpoint_addr().id != connection.remote_id() {
            
        }


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