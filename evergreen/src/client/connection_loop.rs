
use iroh::endpoint::{self, Connection};
use crate::types::*;
use tokio::sync::{broadcast, mpsc};






/// Instruction to be broadcast to all connections
/// 
pub enum Order {
    /// A client *may* connect to new peers whose room ids do not match.
    /// However, when a room update is broadcast, connections with non-matching room ids will be closed.
    UpdateRoom(u64),
    /// An individual connection determines whether to send the packet depending on if the room ids of this client and the peer match
    SendData(Vec<u8>),
}

pub enum ConnectionUpdates {

}



pub struct ConnectionHandle {
    pub broadcast: broadcast::Sender<Order>,
    pub sender: mpsc::Sender<ConnectionUpdates>,
    pub receiver: mpsc::Receiver<ConnectionUpdates>,
}

impl ConnectionHandle {
    
}

pub struct ClientHandle {
    receiver: broadcast::Receiver<Order>,
    sender: mpsc::Sender<ConnectionUpdates>
}


#[derive(thiserror::Error, Debug)]
pub enum ConnectionError {
    #[error(transparent)]
    IncomingError(#[from] IncomingError)
}



//
pub async fn handle_connection(incoming: endpoint::Incoming, handle: ClientHandle) -> Result<(), ConnectionError> {
    
    let (connection, peer_info) = handle_incoming(incoming).await?;





    todo!()
}






pub struct IncomingList {
    pub list: Vec<tokio::task::JoinHandle<Result<(iroh::endpoint::Connection, PeerInfo), IncomingError>>>,
}




#[derive(thiserror::Error, Debug)]
pub enum IncomingError {
    #[error("Task join error")]
    JoinError(#[from] tokio::task::JoinError),

    #[error("Error establishing connection to a new endpoint")]
    ConnectingError(#[from] iroh::endpoint::ConnectingError),
    
    #[error("Connection to outside endpoint failed")]
    ConnectionError(#[from] iroh::endpoint::ConnectionError),

    #[error("Packet received was larger than maximum size")]
    PacketTooBig(#[from] iroh::endpoint::ReadToEndError),

    #[error("Packet was malformed or otherwise could not be parsed")]
    PacketParseError(#[from] protobuf::ParseError),

    #[error("Packet was not of type InitPacket")]
    WrongType,
}

const INIT_PACKET_LIMIT: usize = 1024;



use crate::wire::Packet;

async fn handle_incoming(incoming: iroh::endpoint::Incoming) -> Result<(Connection, PeerInfo), IncomingError> {

    let connection = incoming.await?;

    
    let (send, mut recv) = connection.accept_bi().await?;

    //for new connections, we first wait for the incoming (connecting) peer to send us data about themselves before we respond
    let data = recv.read_to_end(INIT_PACKET_LIMIT).await?;

    let packet = crate::wire::deserialize_packet(&data)?;

    let Packet::Init(init) = packet else {return Err(IncomingError::WrongType)};

    


    todo!()
}

// This is getting to the point where I'm tempted to make a crate just dedicated to futures across vectors
// 
// 
// Not tempted enough.

use std::{collections::HashMap, task::Poll};


impl Future for IncomingList {
    type Output = (usize, Result<(Connection, PeerInfo), IncomingError>);

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        for (index, handle) in self.list.iter_mut().enumerate() {
            if let Poll::Ready(res) = std::pin::Pin::new(handle).poll(cx) {

                // If there's an easier way to do this: please tell me

                let val = match res {
                    Ok(Ok(conn)) => Ok(conn),
                    Ok(Err(err)) => Err(err),
                    Err(err) => Err(err.into()),
                };

                return Poll::Ready((index, val));
            }
        }

        Poll::Pending
    }
}