

mod auth;


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

use core::fmt;
use std::convert::Infallible;

use libp2p::identity;
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
pub enum Error {
    None,
    Multiaddr(libp2p::multiaddr::Error),
    Transport(libp2p::TransportError<std::io::Error>),
    Subscroption(libp2p::gossipsub::SubscriptionError),
    Dial(libp2p::swarm::DialError)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::None => write!(f, "No Error"),
            Error::Multiaddr(error) => write!(f, "{error}"),
            Error::Transport(transport_error) => write!(f, "{transport_error}"),
            Error::Subscroption(subscription_error) => write!(f, "{subscription_error}"),
            Error::Dial(dial_error) => write!(f, "{dial_error}"),
        }
        
    }
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        Self::None
    }
}

impl From<libp2p::multiaddr::Error> for Error {
    fn from(value: libp2p::multiaddr::Error) -> Self {
        Self::Multiaddr(value)
    }
}

impl From<libp2p::TransportError<std::io::Error>> for Error {
    fn from(value: libp2p::TransportError<std::io::Error>) -> Self {
        Self::Transport(value)
    }
}

impl From<libp2p::gossipsub::SubscriptionError> for Error {
    fn from(value: libp2p::gossipsub::SubscriptionError) -> Self {
        Self::Subscroption(value)
    }
}

impl From<libp2p::swarm::DialError> for Error {
    fn from(value: libp2p::swarm::DialError) -> Self {
        Self::Dial(value)
    }
}