// SPDX-License-Identifier: Apache-2.0

pub mod auth;

pub mod error;

mod packet;

pub use packet::*;


// pub type ItemId = i64;
pub type Timestamp = i64;
type PeerString = String;
// pub type ItemType = String;




// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub struct Item {
//     created_at: Timestamp,
//     created_by: PeerString,
//     owner: PeerString,
//     id: ItemId, //hash of timestamp + peerstring
//     item_type: String, //e.g. com.firesidexr.item.stick or com.firesidexr.food.marshmallow
// }





use libp2p::{PeerId, identity::{self, Keypair, PublicKey}};
use serde::{Deserialize, Serialize};

pub struct Identity {
    public_key: identity::PublicKey
}


impl Identity {
    pub fn peer_id(&self) -> identity::PeerId {
        return self.public_key.to_peer_id()
    }
}


/// Stores data related to the public profile of a user on the network.
/// Most of this data can be set freely by a user. Restrictions on this data are applied by other clients.
/// 
/// For our purposes, this is a display name, head, torso, backpack, and primary and accent colors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Default)]
pub struct Avatar {
    pub display_name: String,
    pub head: String,
    pub torso: String,
    pub backpack: String,
    pub primary_color: String,
    pub accent_color: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Peer {
    pub identity: String,
    pub avatar: Avatar,
    pub passports: Vec<String>,
}





/// Enum for sending commands to a network instance through a NetworkHandle
pub enum Command {
    /// Broadcast data to the network
    SendData(PacketData),

    /// Take some action as the client (such as disconnecting)
    ClientCommand(ClientCommand),

    /// Talk directly to the connected server. Unused in p2p settings
    ServerCommand(ServerCommand),
}

pub enum ClientCommand {
    Disconnect,
}

pub enum ServerCommand {
    DisconnectPeer(PeerId),
    BanPeer(PeerId),
    KickPeer(PeerId),
}

/// Enum for getting responses from a network instance through a NetworkHandle
#[derive(Debug)]
pub enum Response {
    Server(ServerResponse),
    Client(ClientResponse),
    Network(NetworkUpdate)
}

#[derive(Debug)]
pub struct ClientResponse {
    pub peer: libp2p::PeerId,
    pub data: PacketData
}

#[derive(Debug)]

pub struct ServerResponse {

}

#[derive(Debug)]
pub enum NetworkUpdate {
    AliveWithAddr(String),
    NewPeer(PeerId),
    PeerDisconnected(PeerId),
    Disconnected,
}





use tokio::sync::mpsc;

/// Used to pass messages back and forth to the network thread
pub struct NetworkHandle {
    outgoing_commands: mpsc::Sender<Command>,
    incoming_events: mpsc::Receiver<Response>
}

impl NetworkHandle {

    pub fn new() -> (Self, mpsc::Receiver<Command>, mpsc::Sender<Response>) {
        let (outgoing_commands, incoming_commands) = mpsc::channel::<Command>(64);
        let (outgoing_events, incoming_events) = mpsc::channel::<Response>(256);

        (Self { outgoing_commands, incoming_events }, incoming_commands, outgoing_events)
    }

    pub fn is_closed(&self) -> bool {
        self.outgoing_commands.is_closed() || self.incoming_events.is_closed()
    }

    pub fn send_data_blocking(&self, data: PacketData) {
        self.send_command_blocking(Command::SendData(data));
    }

    pub async fn send_data(&self, data: PacketData) {
        self.send_command(Command::SendData(data)).await
    }

    pub fn send_command_blocking(&self, command: Command) {
        let _ = self.outgoing_commands.blocking_send(command);
    }

    pub async fn send_command(&self, command: Command) {
        let _ = self.outgoing_commands.send(command).await;
    }

    pub fn get_event_blocking(&mut self) -> Option<Response> {
        self.incoming_events.try_recv().ok()
    }

    pub async fn get_event(&mut self) -> Option<Response> {
        self.incoming_events.recv().await
    }

}


