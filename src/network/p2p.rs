// SPDX-License-Identifier: Apache-2.0
use libp2p::swarm::NetworkBehaviour;
use libp2p::request_response;

use crate::types::PacketData;




#[derive(NetworkBehaviour)]
#[behaviour(to_swarm="PeerEvent")]
pub struct PeerBehaviour {
    pub request: libp2p::request_response::cbor::Behaviour<PacketData, ()>
}

#[derive(Debug)]
pub enum PeerEvent {
    Request(request_response::Event<PacketData, ()>)
}

impl From<request_response::Event<PacketData, ()>> for PeerEvent {
    fn from(value: request_response::Event<PacketData, ()>) -> Self {
        Self::Request(value)
    }
}