use futures::StreamExt;
use libp2p::identity::Keypair;
use libp2p::request_response::ProtocolSupport;
use libp2p::swarm::NetworkBehaviour;
use libp2p::request_response;
use libp2p::gossipsub;
use libp2p::swarm::SwarmEvent;
use libp2p::StreamProtocol;


use crate::types::*;

use tokio::sync::mpsc;


// struct ServerConn {

// }

// impl ServerConn {
//     pub async fn send(req: Request) {

//     }





// }


pub struct ClientResponse {
    pub peer: libp2p::PeerId,
    pub data: PacketData
}

pub struct ServerResponse {

}

pub enum NetworkUpdate {
    AliveWithAddr(String),
    Disconnected,
}


pub enum Response {
    Server(ServerResponse),
    Client(ClientResponse),
    Network(NetworkUpdate)
}



pub fn run(key: Keypair, server: bool, addr: libp2p::Multiaddr) -> (tokio::task::JoinHandle<Result<(), Error>>, mpsc::Sender<PacketData>, mpsc::Receiver<Response>) {

    let (request_sender, mut request_reciever) = mpsc::channel::<PacketData>(64);
    let (response_sender, response_reciever) = mpsc::channel::<Response>(128);


    let handle = tokio::spawn(async move {



        let mut swarm = libp2p::SwarmBuilder::with_existing_identity(key)
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

        if server {
            swarm.listen_on(addr)?;
        } else {
            swarm.dial(addr)?;
        }

        


        let chat = gossipsub::IdentTopic::new("chat");
        swarm.behaviour_mut().gossipsub.subscribe(&chat)?;


        


        loop {


            match request_reciever.recv().await {
                Some(_request) => { 
                    todo!() 
                }
                None => ()
            }


            match swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => {
                    let _ = response_sender.send(Response::Network(NetworkUpdate::AliveWithAddr(address.to_string()))).await;
                },
                SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                    swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                }
                SwarmEvent::Behaviour(Event::GossipsubNotSupported(peer)) => {
                    let _ = swarm.disconnect_peer_id(peer);
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
                    let _ = response_sender.send(Response::Client(ClientResponse { peer: peer_id, data })).await;
                }
                SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                    println!("Closed: {peer_id} with cause: {cause:?}")
                }
                _ => {}
            }
        }
        
    });

    return (handle, request_sender, response_reciever)
}








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

