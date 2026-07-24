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

mod serialization {
    include!(concat!(env!("OUT_DIR"), "/protobuf_generated/generated.rs"));
}

use protobuf::{Parse, Serialize};



pub struct InitialPayload {
    pub ticket: String,
    pub passports: Vec<String>,
    pub other_peers: Vec<String>,

    // Technically these can be "none" by being empty but we want to be explicit with how we model our data
    pub room_id: Option<u64>,
    pub data: Option<Vec<u8>>,
}

pub enum Packet {
    Init(InitialPayload),
    Data(Vec<u8>),

}


use serialization::base_packet::PacketOneof;

use crate::types;

/// Internal function
/// 
/// Used to deserialize packets :P
pub(crate) fn deserialize_packet(packet: &[u8]) -> Result<Packet, protobuf::ParseError> {
    let base = serialization::BasePacket::parse(packet.into())?;

    match base.packet() {
        // At the moment we're just reusing the protobuf error as a generic parse error. 
        // Should the parsing get stricter we should implement a custom error for this case.
        PacketOneof::not_set(_) => Err(protobuf::ParseError),
        PacketOneof::UpdatePacket(_) => todo!(),
        PacketOneof::DataPacket(data_packet) => Ok(Packet::Data(data_packet.data().to_vec())),
        PacketOneof::InitPacket(init_packet) => {
            Ok(Packet::Init(InitialPayload 
                { 
                    ticket: init_packet.ticket().to_string(),
                    passports: init_packet.passports().into_iter().map(|x| {x.jwt().to_string()}).collect(), 
                    other_peers: init_packet.other_peers().into_iter().map(|x|{x.to_string()}).collect(), 
                    room_id: if init_packet.has_room_id()  {Some(init_packet.room_id())} else {None}, 
                    data: if init_packet.has_data() { Some(init_packet.data().to_vec()) } else {None}
                }
            ))
        },
    }
}

pub(crate) fn serialize_initial_payload(
    ticket: &str,
    passports: &[crate::types::Passport],
    other_peers: &[String],
    room_id: Option<u64>,
    data: &[u8],
) -> Result<Vec<u8>, protobuf::SerializeError> {
    let mut packet = serialization::InitPacket::new();

    packet.set_ticket(ticket);

    // God is this library a pain to work with
    passports.iter().map(map_passport_to_protobuf).for_each(|p| packet.passports_mut().push(p.as_view()));

    other_peers.iter().for_each(|ticket| packet.other_peers_mut().push(ticket));
    
    if let Some(id) = room_id {packet.set_room_id(id)};
    packet.set_data(data);
    packet.serialize()
}


fn map_passport_to_protobuf(p: &types::Passport) -> serialization::Passport {
    let jwt = p.to_string();
    let mut passport = serialization::Passport::new();
    passport.set_jwt(jwt);
    passport
}