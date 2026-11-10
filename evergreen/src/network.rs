// Copyright (c) 2026 Anders Olsen
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

use crate::rooms::Room;

use crate::types::Peer;

use tokio::sync::oneshot;

#[derive(Debug)]
pub struct Request {
    inner_sender: oneshot::Sender<bool>
}

impl Request {
    fn new() -> (Self, oneshot::Receiver<bool>) {
        let (send, recv) = oneshot::channel::<bool>();
        (Self{inner_sender: send}, recv)
    }

    pub fn allow(self) {
        let _ = self.inner_sender.send(true);
    }

    pub fn deny(self) {
        let _ = self.inner_sender.send(false);
    }
}





#[derive(Debug)]
pub enum NetworkEvent {
    NewRoomJoined(Room),

    JoinRequested(Peer, Request),
    StatusRequested(Peer, Request),

    PeerJoined(Peer),
    PeerLeft(Peer),

    Packet(Peer, Vec<u8>),


}