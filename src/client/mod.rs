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

use std::collections::HashMap;

use futures::StreamExt;
use libp2p::autonat::{InboundFailure, OutboundFailure};
use libp2p::identity::Keypair;
use libp2p::request_response::{InboundRequestId, OutboundRequestId, ProtocolSupport};
use libp2p::{request_response::Message, swarm::NetworkBehaviour};
use libp2p::{PeerId, request_response};

use crate::types::*;

use tokio::sync::mpsc;






#[derive(NetworkBehaviour)]
#[behaviour(to_swarm="Event")]
pub struct Behaviour {
    pub request: libp2p::request_response::cbor::Behaviour<RawPacket, ()>

}

#[derive(Debug)]
pub enum Event {
    //request response
    Message(PeerId, ConnectionId, Message<RawPacket, ()>),
    OutboundFailure(PeerId, ConnectionId, OutboundRequestId, OutboundFailure),
    InboundFailure(PeerId, ConnectionId, InboundRequestId, InboundFailure),
    ResponseSent(PeerId, ConnectionId, InboundRequestId),


}

impl From<request_response::Event<RawPacket, ()>> for Event {
    fn from(value: request_response::Event<RawPacket, ()>) -> Self {
        match value {
            request_response::Event::Message { peer, connection_id, message } => 
                Self::Message(peer, connection_id, message),
            request_response::Event::OutboundFailure { peer, connection_id, request_id, error } => 
                Self::OutboundFailure(peer, connection_id, request_id, error),
            request_response::Event::InboundFailure { peer, connection_id, request_id, error } => 
                Self::InboundFailure(peer, connection_id, request_id, error),
            request_response::Event::ResponseSent { peer, connection_id, request_id } => 
                Self::ResponseSent(peer, connection_id, request_id),
        }
    }
}








pub struct Config {
    pub sign_packets: bool,
}


pub struct Network {

    config: Config,

    keypair: Keypair,

    identity: Vec<u8>,

    sequence_number: u64,

    peers: HashMap<libp2p::PeerId, Peer>,

    incoming_commands: mpsc::Receiver<Command>,
    outgoing_events: mpsc::Sender<Response>,

    swarm: libp2p::Swarm<Behaviour>,
}



impl Network {

    pub fn new(keypair: Option<Keypair>, config: Config) -> Result<(Self, NetworkHandle), Error> {

        let keypair = keypair.unwrap_or(Keypair::generate_ed25519());

        let (net_handle, incoming_commands, outgoing_events) = NetworkHandle::new();

        let swarm = libp2p::SwarmBuilder::with_existing_identity(keypair.clone())
        .with_tokio().with_quic()
        .with_behaviour(|_| Behaviour{
            request: request_response::cbor::Behaviour::<RawPacket, ()>::new(
                [(
                    StreamProtocol::new("/evergreen-1.0"), ProtocolSupport::Full,
                )], 
                request_response::Config::default(),
            ),
        })?.build();


        
        Ok((
            Self{
                config,
                keypair: keypair.clone(),
                identity: keypair.public().to_peer_id().to_bytes(),
                sequence_number: rand::random(),
                peers: HashMap::new(),
                incoming_commands,
                outgoing_events,
                swarm,
            },
            net_handle
        ))
    }








}

use libp2p::swarm::{SwarmEvent, *};

use crate::types::{Response, NetworkUpdate};

impl Network {
    async fn handle_packet(&mut self, packet: ValidPacket) {


        if let PacketData::SetAvatar(avatar_data) = packet.data{
            self.peers.get_mut(&packet.source).unwrap_or(&mut Peer::default()).avatar = avatar_data.clone()
        }

        //let _ = self.outgoing_events.send(Response::Client(ClientResponse { peer: peer, data: request.data})).await;
        }

    async fn handle_event(&mut self, event: SwarmEvent<Event>) {

        println!("{:?}", event);
        match event {
            SwarmEvent::NewListenAddr { address, .. } => {

                let _ = self.outgoing_events.send(Response::Network(NetworkUpdate::AliveWithAddr(address.to_string()))).await;
            }
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                self.peers.insert(peer_id.clone(), Peer::default());
                
                let _ = self.outgoing_events.send(Response::Network(NetworkUpdate::NewPeer(peer_id))).await;
            }
            SwarmEvent::Behaviour(Event::Message(peer, _connection_id, message)) => {
                
                match message {
                    Message::Response { .. } => {}, //This doesn't exist. It's a figment of your imagination.

                    Message::Request {request, .. } => {

                        match PacketData::try_from(request) {
                            Ok(packet_data) => self.handle_packet(packet_data).await,
                            Err(err) => println!("{:?}", err),
                        }
                    },
                }
                
            }
            
            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                let _ = self.outgoing_events.send(Response::Network(NetworkUpdate::PeerDisconnected(peer_id))).await;
                //println!("Closed: {peer_id} with cause: {cause:?}")
            }
            _swarm_event => {
                //println!("Internal: {swarm_event:?}")
            }
        }
    }


    async fn handle_command(&mut self, command: Command) {
        match command {
            Command::SendData(data) => {

                self.sequence_number += 1;
                
                let packet = match self.config.sign_packets {
                    // If we're signing packets, add a signature
                    true => match RawPacket::new_signed(data, self.sequence_number, &self.keypair, self.keypair.public().to_peer_id()) {
                        
                        Ok(packet) => packet,
                        Err(_) => {return},
                    },
                    // If we're not, just make a boring old regular packet
                    false => {
                        
                        //They boiler my plate till I
                        let (packet_type, data) = data.as_bytes();
                        
                        RawPacket {
                            packet_type,
                            data,
                            sequence_number: self.sequence_number,
                            source: self.identity.clone(),
                            signature: None
                    }},
                };

                //Send it off over the network
                for peer in self.peers.keys() {
                    self.swarm.behaviour_mut().request.send_request(peer, packet.clone());
                }
            }
            _ => ()
        }
    }
    
    pub async fn run(&mut self) {
        loop {
            tokio::select! {
                event = self.swarm.select_next_some() => {
                    self.handle_event(event).await
                }
                command = self.incoming_commands.recv() => match command {
                    Some(data) => self.handle_command(data).await,
                    None => return
                }
            }
        }
    }
}