// Copyright (c) 2025 Anders Olsen
//
// Permission is hereby granted, free of charge, to any person obtaining 
// a copy of this software and associated documentation files (the "Software"), 
// to deal in the Software without restriction, including without limitation the 
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is 
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in 
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, 
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, 
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS 
// OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN 
// AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH 
// THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use serde::{Deserialize, Serialize};
use crate::types::*;

#[derive(Debug, Clone, PartialEq)]
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
    pub fn as_bytes(&self) -> (Vec<u8>, Vec<u8>) {

        match self {
            PacketData::Message(message) => 
                return ("message".as_bytes().to_vec(), message.as_bytes().to_vec()),
            PacketData::Voice(voice_data) =>
                return ("voice".as_bytes().to_vec(), voice_data.clone()),
            PacketData::Movement(_) => todo!(),
            PacketData::AddPassport(_) => todo!(),
            PacketData::SetAvatar(_) => todo!(),
        }
    }

    pub fn from_bytes(packet_type: impl Into<Vec<u8>>, data: impl Into<Vec<u8>>) -> Result<Self, Error> {

        let packet_type: &str = &String::from_utf8(packet_type.into())?;
        let data: Vec<u8> = data.into();

        Ok( match packet_type {
            "message" => PacketData::Message(String::from_utf8(data)?),
            _ => todo!(),
        } )
    }
}

impl TryFrom<RawPacket> for PacketData {

    type Error = Error;
    
    fn try_from(value: RawPacket) -> Result<PacketData, Error> {
        PacketData::from_bytes(value.packet_type, value.data)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawPacket {
    
    pub packet_type: Vec<u8>,

    pub data: Vec<u8>,

    pub sequence_number: u64,

    pub source: Vec<u8>,

    pub signature: Option<Vec<u8>>,
}


impl RawPacket {
    pub fn new_signed(data: PacketData, sequence_number: u64, key: &Keypair, source: PeerId) -> Result<Self, error::Error> {

        let (packet_type, data) = data.as_bytes();
        
        let mut message: Vec<u8> = Vec::new();

        message.extend(&packet_type);

        message.extend(&data);

        message.extend_from_slice(&sequence_number.to_be_bytes());

        message.extend(source.to_bytes());

        match key.sign(&message) {
            Ok(signature) => Ok( Self { packet_type, data, sequence_number, source: source.to_bytes(), signature: Some(signature)}),
            Err(err) => Err(err.into()),
        }
    }
}


#[derive(Debug, Clone)]
pub struct ValidPacket {
    pub data: PacketData,

    pub source: PeerId,

    pub sequence_number: u64,

    pub verified: bool,
}

impl ValidPacket {
    pub fn new(packet: RawPacket, key: PublicKey, peer_id: PeerId) -> Option<Self> {

        let Ok(data) = PacketData::from_bytes(packet.packet_type.clone(), packet.data.clone()) else {return None};
        let sequence_number = packet.sequence_number;

        match packet.signature {

            // No signature. Skip verification and set the flag to false
            None => Some( Self {
                data,
                source: peer_id,
                sequence_number,
                verified: false,
            }),
            //Signature is present. Attempt to verify it.
            Some(signature) => {

                let mut message: Vec<u8> = Vec::new();

                message.extend(&packet.packet_type);

                message.extend(&packet.data);

                message.extend_from_slice(&packet.sequence_number.to_be_bytes());

                message.extend(packet.source);

                let verified = key.verify(&message, &signature);

                Some(
                    Self {
                        data,
                        source: peer_id,
                        sequence_number,
                        verified,
                    }
                )
            },
        }
    }
}