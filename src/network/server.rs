// SPDX-License-Identifier: Apache-2.0
use futures::StreamExt;
use libp2p::gossipsub::IdentTopic;
use libp2p::identity::Keypair;
use libp2p::request_response::ProtocolSupport;
use libp2p::swarm;
use libp2p::swarm::NetworkBehaviour;
use libp2p::request_response;
use libp2p::gossipsub;
use libp2p::swarm::SwarmEvent;
use libp2p::Multiaddr;
use libp2p::StreamProtocol;


use crate::types::*;

use tokio::sync::mpsc;


// struct ServerConn {

// }

// impl ServerConn {
//     pub async fn send(req: Request) {

//     }





// }


pub struct NetworkHandle {
    outgoing_commands: mpsc::Sender<PacketData>,
    incoming_events: mpsc::Receiver<Response>
}

impl NetworkHandle {

    pub fn is_closed(&self) -> bool {
        self.outgoing_commands.is_closed() || self.incoming_events.is_closed()
    }

    pub fn send_command_blocking(&self, command: PacketData) {
        let _ = self.outgoing_commands.blocking_send(command);
    }

    pub async fn send_command(&self, command: PacketData) {
        let _ = self.outgoing_commands.send(command).await;
    }

    pub fn get_event_blocking(&mut self) -> Option<Response> {
        self.incoming_events.try_recv().ok()
    }

    pub async fn get_event(&mut self) -> Option<Response> {
        self.incoming_events.recv().await
    }

}






pub struct Network {
    keypair: libp2p::identity::Keypair,

    incoming_commands: mpsc::Receiver<PacketData>,
    outgoing_events: mpsc::Sender<Response>,

    chat: IdentTopic,

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
        let (outgoing_commands, incoming_commands) = mpsc::channel::<PacketData>(64);
        let (outgoing_events, incoming_events) = mpsc::channel(128);
 
        let mut swarm = libp2p::SwarmBuilder::with_existing_identity(key.clone())
        .with_tokio().with_quic()
        .with_behaviour(|key| Behaviour{
            request: request_response::cbor::Behaviour::<ServerUpdate, ()>::new(
                [(
                    StreamProtocol::new("/evergreen"), ProtocolSupport::Full,
                )], 
                request_response::Config::default(),
            ),
            gossipsub: gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(key.clone()), 
                gossipsub::Config::default()
            ).expect("No Gossipsub")
        })?
        .build();

        if is_server {
            swarm.listen_on(addr)?;
        } else {
            swarm.dial(addr)?;
        }

        let chat = gossipsub::IdentTopic::new("chat");
        swarm.behaviour_mut().gossipsub.subscribe(&chat)?;



        let server = Self{
            keypair: key,
            incoming_commands,
            outgoing_events,
            chat: IdentTopic::new("chat"),
            swarm,
        };

        let handle = NetworkHandle{
            outgoing_commands,
            incoming_events,
        };

        return Ok((server, handle))
    }





}


impl Network {
    async fn handle_event(&mut self, event: swarm::SwarmEvent<Event>) {
        match event {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Internal: Alive with addr");
                let _ = self.outgoing_events.send(Response::Network(NetworkUpdate::AliveWithAddr(address.to_string()))).await;
            },
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                println!("Internal: Connection established");
            }
            SwarmEvent::Behaviour(Event::GossipsubNotSupported(peer)) => {
                let _ = self.swarm.disconnect_peer_id(peer);
            }
            SwarmEvent::Behaviour(Event::Request(_peer, _packet)) => {
                todo!()
                // let peers: Vec<libp2p::PeerId> = swarm.connected_peers().copied().collect();
                
                // for other_peer in peers {
                //     if peer == other_peer { continue }
                    
                //     swarm.behaviour_mut().request.send_request(&other_peer, packet.clone());
                // }

            }
            SwarmEvent::Behaviour(Event::Message(peer_id,data )) => {
                let _ = self.outgoing_events.send(Response::Client(ClientResponse { peer: peer_id, data })).await;
            }
            SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                println!("Closed: {peer_id} with cause: {cause:?}")
            }
            _ => {
                println!("Internal: None")
            }
        }
    }


    async fn handle_command(&mut self, command: PacketData) {
        match command {
            PacketData::Message(msg) => {
                let _ = self.swarm.behaviour_mut().gossipsub.publish(self.chat.clone(), msg);
            },
            PacketData::Movement(_) => todo!(),
            PacketData::AddPassport(_) => todo!(),
            PacketData::UpdateAvatar(_avatar) => todo!(),
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
    pub gossipsub: libp2p::gossipsub::Behaviour,
}

#[derive(Debug)]
pub enum Event {
    Gossip(gossipsub::Event),
    Subscribed(libp2p::PeerId, gossipsub::TopicHash),
    Unsubscribed(libp2p::PeerId, gossipsub::TopicHash),
    Message(libp2p::PeerId, PacketData),
    GossipsubNotSupported(libp2p::PeerId),
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

impl From<gossipsub::Event> for Event {
    fn from(value: gossipsub::Event) -> Self {
        match value {
            gossipsub::Event::Message { propagation_source, message , ..} => {
                match PacketData::from_bytes(message.data) {
                    Ok(packet) => Self::Message(propagation_source, packet),
                    Err(_) => Self::Other
                }
            },
            gossipsub::Event::Subscribed { peer_id, topic } => {
                Self::Subscribed(peer_id, topic)
            },
            gossipsub::Event::Unsubscribed { peer_id, topic } => {
                Self::Unsubscribed(peer_id, topic)
            },
            gossipsub::Event::GossipsubNotSupported { peer_id } => {
                Self::GossipsubNotSupported(peer_id)
            },
            _ => Self::Other
        }
    }
}





#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ServerUpdate {
    Peers(Vec<Peer>)
}

