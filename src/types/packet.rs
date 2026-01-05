// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum PacketData {
    Message(String),
    Voice(Vec<u8>),
    Movement([[f32; 3]; 12]),
    // Puppet(ItemId, [[f32; 3]; 4]),
    // GiveItem(ItemId, PeerString),
    AddPassport(String),
    SetAvatar(Avatar)
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SignedPacket {
    
    pub data: PacketData,

    pub sequence_number: u64,

    pub source: Vec<u8>,

    pub signature: Vec<u8>,
}


impl SignedPacket {
    pub fn new(data: PacketData, sequence_number: u64, key: Keypair, source: PeerId) -> Result<SignedPacket, error::Error> {

        let mut message = data.as_bytes();

        message.extend_from_slice(&sequence_number.to_be_bytes());

        message.extend(source.to_bytes());

        match key.sign(&message) {
            Ok(signature) => Ok(SignedPacket { data, sequence_number, source: source.to_bytes(), signature}),
            Err(err) => Err(err.into()),
        }
    }
}


#[derive(Debug, Clone)]
pub struct VerifiedMessage {
    pub data: PacketData,

    pub source: PeerId,
}


impl VerifiedMessage {
    pub fn new(packet: SignedPacket, key: PublicKey, peer_id: PeerId) -> Option<Self> {

        let mut message = packet.data.as_bytes();

        message.extend_from_slice(&packet.sequence_number.to_be_bytes());

        message.extend(packet.source);

        
        if key.verify(&message, &packet.signature) {
            Some( Self {
                data: packet.data,
                source: peer_id,
            } )
        } else {
            None
        }
    }
}