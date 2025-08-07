// SPDX-License-Identifier: Apache-2.0

pub mod auth;

pub mod error;


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PacketData {
    Message(String),
    Movement([Vector3; 12]),
    AddPassport(String),
    UpdateAvatar(Avatar)
}

impl PacketData {
    pub fn as_bytes(&self) -> Vec<u8> {
        postcard::to_allocvec(self).unwrap()
    }

    pub fn from_bytes(bytes: impl Into<Vec<u8>>) -> postcard::Result<Self> {
        let bytes: Vec<u8> = bytes.into();

        postcard::from_bytes(&bytes)
    }
}


pub enum Command {
    //broadcast some data to the network
    SendData(PacketData),

    ClientCommand(ClientCommand),

    //unused in p2p settings
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





use libp2p::{identity, PeerId};
use serde::{Deserialize, Serialize};


pub struct Identity {
    public_key: identity::PublicKey
}

impl Identity {
    pub fn peer_id(&self) -> identity::PeerId {
        return self.public_key.to_peer_id()
    }


}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Avatar {
    pub head: String,
    pub torso: String,
    pub backpack: String,
    pub primary_color: String,
    pub accent_color: String,
}

pub struct Provider {
    pub provider: String, //com.firesidexr.client
    pub public_keys: Vec<identity::PublicKey>,
    pub revoked_jwts: Vec<i64>,
}

pub struct ProviderList {
    _list: Vec<Provider>
}


impl ProviderList {

    pub fn create_passport(&self, _jwt: String) -> Result<auth::Passport, auth::PassportError> {


        todo!()
    }

}



#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Peer {
    pub identity: String,
    pub avatar: Avatar,
    pub passports: Vec<auth::Passport>,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Vector3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
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
    Disconnected,
}


#[derive(Debug)]
pub enum Response {
    Server(ServerResponse),
    Client(ClientResponse),
    Network(NetworkUpdate)
}