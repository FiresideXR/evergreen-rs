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

use evergreen::prelude::*;


const SERVER_LIST: ServerList<[&'static str; 1]> = ServerList(["localhost"]);

const APP_ID: &str = "com.dungenrobot.chatroom";





// Do not use this in production. 
// This thing panics at like every kind of failure. It has zero proper error handling
async fn quickstart() -> (Client, Identity) {
    let identity = Identity::generate();
    let client_config = ClientConfig::new(identity.clone(), RoomTransition::Load);
    let client = Client::create_and_start(client_config).await?;

    let server_list = SERVER_LIST;
    
    let room = SERVER_LIST.find_or_host(APP_ID).await.unwrap();

    let peer_list = SERVER_LIST.find_peers(room.room_id()).await.unwrap();

    client.connect_to_room(room, SERVER_LIST);

    SERVER_LIST.post_status(&identity, room.room_id());

    (client, identity)
}



#[tokio::main]
async fn main() -> anyhow::Result<()>{

    let (client, identity) = quickstart().await;

    tokio::select! {
        _ = client.stopped().await => {
            return Ok(())
        },

        network_event = client.get_next_event().await => {
            match network_event {

                NetworkEvent::NewRoomJoined(room) => {

                },

                NetworkEvent::JoinRequested(request, peer) => {
                    
                },
                NetworkEvent::PeerJoined(peer) => {

                    print!("Peer: {} joined room: {} with public key: {:?}", peer.name, peer.room, peer.identity)
                },
                NetworkEvent::PeerLeft(peer) => {

                },
                NetworkEvent::Packet(sending_peer, data) => {

                },
                NetworkEvent::StatusRequested(peer, request) => {

                }

                //???
                // NetworkEvent::FriendshipRequested(peer) => {

                // },
                // NetworkEvent::FriendshipAccepted(peer) => {

                // },
            }



            print!("{:?}", network_event)
        }
    }

    Ok(())
}