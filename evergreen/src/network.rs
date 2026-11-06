
use crate::rooms::Room;

use crate::types::Peer;

pub struct Request;

impl Request {
    pub fn allow(self) {

    }

    pub fn deny(self) {

    }
}






pub enum NetworkEvent {
    NewRoomJoined(Room),

    JoinRequested(Peer, Request),
    StatusRequested(Peer, Request),

    PeerJoined(Peer),
    PeerLeft(Peer),

    Packet(Peer, Vec<u8>),


}