use std::collections::HashMap;

// SPDX-License-Identifier: Apache-2.0
use futures::StreamExt;
use libp2p::autonat::InboundProbeEvent;
//use libp2p::gossipsub::{self, IdentTopic};
use libp2p::identity::Keypair;
use libp2p::request_response::{self, ProtocolSupport};
use libp2p::swarm::{self, NetworkBehaviour, SwarmEvent};
use libp2p::{Multiaddr, autonat};
use libp2p::StreamProtocol;


use crate::types::NetworkHandle;
use crate::types::{*, error::Error};



use tokio::sync::mpsc;






pub struct Network {
    keypair: libp2p::identity::Keypair,

    peers: HashMap<libp2p::PeerId, Peer>,

    incoming_commands: mpsc::Receiver<Command>,
    outgoing_events: mpsc::Sender<Response>,

    //chat: IdentTopic,

    swarm: libp2p::Swarm<Behaviour>,
}

impl Network {

    pub fn new_server(key: Keypair, addr: Multiaddr) -> Result<(Self, NetworkHandle), Error> {
        Self::new(key, addr, true)
    }


    pub fn new_client(key: Keypair, addr: Multiaddr) -> Result<(Self, NetworkHandle), Error> {
        Self::new(key, addr, false)
    }


    fn new(key: Keypair, addr: Multiaddr, is_server: bool) -> Result<(Self, NetworkHandle), Error> {

        let (net_handle, incoming_commands, outgoing_events) = NetworkHandle::new();

        // let (outgoing_commands, incoming_commands) = mpsc::channel::<Command>(64);
        // let (outgoing_events, incoming_events) = mpsc::channel(256);
 
        let mut swarm = libp2p::SwarmBuilder::with_existing_identity(key.clone())
        .with_tokio().with_quic()
        .with_behaviour(|key| Behaviour{
            request: request_response::cbor::Behaviour::<ServerUpdate, ()>::new(
                [(
                    StreamProtocol::new("/evergreen"), ProtocolSupport::Full,
                )], 
                request_response::Config::default(),
            ),
            autonat: autonat::Behaviour::new(key.public().to_peer_id(), autonat::Config::default()),
            // gossipsub: gossipsub::Behaviour::new(
            //     gossipsub::MessageAuthenticity::Signed(key.clone()), 
            //     gossipsub::Config::default()
            // ).expect("No Gossipsub")
        })?
        .build();

        if is_server {
            swarm.listen_on(addr)?;
        } else {
            swarm.dial(addr)?;
        }

        //let chat = gossipsub::IdentTopic::new("chat");
        //swarm.behaviour_mut().gossipsub.subscribe(&chat)?;



        let server = Self{
            keypair: key,
            incoming_commands,
            outgoing_events,
            peers: HashMap::new(),
            //chat: IdentTopic::new("chat"),
            swarm,
        };

        // let handle = NetworkHandle{
        //     outgoing_commands,
        //     incoming_events,
        // };

        return Ok((server, net_handle))
    }

}


impl Network {
    async fn handle_event(&mut self, event: swarm::SwarmEvent<Event>) {

        println!("{:?}", event);
        match event {
            SwarmEvent::NewListenAddr { address, .. } => {

                
                //println!("Internal: Alive with addr");
                let _ = self.outgoing_events.send(Response::Network(NetworkUpdate::AliveWithAddr(address.to_string()))).await;
            }
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                self.peers.insert(peer_id.clone(), Peer::default());
                //self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                let _ = self.outgoing_events.send(Response::Network(NetworkUpdate::NewPeer(peer_id))).await;
            }
            // SwarmEvent::Behaviour(Event::GossipsubNotSupported(peer)) => {
            //     let _ = self.swarm.disconnect_peer_id(peer);
            // }
            SwarmEvent::Behaviour(Event::Request(_peer, _packet)) => {
                todo!()
            }
            
            SwarmEvent::Behaviour(Event::Message(peer_id,data )) => {
                if let PacketData::SetAvatar(avatar_data) = &data {
                    self.peers.get_mut(&peer_id).unwrap_or(&mut Peer::default()).avatar = avatar_data.clone()
                }
                let _ = self.outgoing_events.send(Response::Client(ClientResponse { peer: peer_id, data })).await;
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
            Command::SendData(data) => match &data {
                PacketData::Message(..) => {
                    //let _ = self.swarm.behaviour_mut().gossipsub.publish(self.chat.clone(), data.as_bytes());
                },
                _ => ()
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







/// Behavior for a hub-spoke evergreen network.
#[derive(NetworkBehaviour)]
#[behaviour(to_swarm="Event")]
pub struct Behaviour {
    pub request: libp2p::request_response::cbor::Behaviour<ServerUpdate, ()>,
    pub autonat: libp2p::autonat::v1::Behaviour,
    //pub gossipsub: libp2p::gossipsub::Behaviour,
}

#[derive(Debug)]
pub enum Event {
    //Gossip(gossipsub::Event),
    //Subscribed(libp2p::PeerId, gossipsub::TopicHash),
    //Unsubscribed(libp2p::PeerId, gossipsub::TopicHash),
    //GossipsubNotSupported(libp2p::PeerId),
    InboundProbe(autonat::InboundProbeEvent),
    OutboundProbe(autonat::OutboundProbeEvent),
    StatusChanged(autonat::NatStatus, autonat::NatStatus),
    Message(libp2p::PeerId, PacketData),
    Request(libp2p::PeerId, ServerUpdate),
    Other
}

impl From<request_response::Event<ServerUpdate, ()>> for Event {
    fn from(value: request_response::Event<ServerUpdate, ()>) -> Self {
        match value {
            request_response::Event::Message { peer, message , ..} => {
                match message {
                    request_response::Message::Request {request, .. } => Self::Request(peer, request),
                    request_response::Message::Response { .. } => Self::Other,
                }
            },
            _ => Self::Other
        }
    }
}

impl From<autonat::Event> for Event {
    fn from(value: autonat::Event) -> Self {
        match value {
            autonat::Event::InboundProbe(inbound_probe_event) => Self::InboundProbe(inbound_probe_event),
            autonat::Event::OutboundProbe(outbound_probe_event) => Self::OutboundProbe(outbound_probe_event),
            autonat::Event::StatusChanged { old, new } => Self::StatusChanged(old, new),
        }
    }
}

// impl From<gossipsub::Event> for Event {
//     fn from(value: gossipsub::Event) -> Self {
//         match value {
//             gossipsub::Event::Message {message , ..} => {
//                 if message.source.is_none() {return Self::Other}
//                 match PacketData::from_bytes(message.data) {
//                     Ok(packet) => Self::Message(message.source.unwrap(), packet),
//                     Err(_) => {println!("failed to parse message data"); Self::Other}
//                 }
//             },
//             gossipsub::Event::Subscribed { peer_id, topic } => {
//                 Self::Subscribed(peer_id, topic)
//             },
//             gossipsub::Event::Unsubscribed { peer_id, topic } => {
//                 Self::Unsubscribed(peer_id, topic)
//             },
//             gossipsub::Event::GossipsubNotSupported { peer_id } => {
//                 Self::GossipsubNotSupported(peer_id)
//             },
//             gossip => Self::Gossip(gossip)
//         }
//     }
// }





#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ServerUpdate {
    Peers(Vec<Peer>)
}

