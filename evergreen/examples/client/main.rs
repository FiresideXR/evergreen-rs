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


const SERVER_LIST: [&'static str; 1] = ["localhost"];

const APP_ID: &str = "com.dungenrobot.chatroom";





// Do not use this in production. 
// This thing panics at like every kind of failure. It has zero proper error handling
async fn quickstart() -> (Client, Identity) {
    let identity = Identity::generate();
    let client_config = ClientConfig::new(identity.clone(), RoomTransition::Load);
    let client = Client::create_and_start(client_config).await.unwrap();

    let room = SERVER_LIST.find_or_host(APP_ID).await.unwrap();

    let peer_list = SERVER_LIST.find_peers(room.room_id()).await.unwrap();

    client.connect_to_peers(&room, &peer_list).await;

    client.connect_to_room(&room, &SERVER_LIST).await;

    SERVER_LIST.post_status(&identity, room.room_id()).await;

    (client, identity)
}



#[tokio::main]
async fn main() -> anyhow::Result<()>{

    let (client, identity) = quickstart().await;

    tokio::select! {
        _ = client.stopped() => {
            return Ok(())
        },

        

        network_event = client.get_next_event() => {

            print!("{:?}", &network_event);

            match network_event {

                NetworkEvent::NewRoomJoined(room) => {

                },

                NetworkEvent::JoinRequested(request, peer) => {
                    
                },
                NetworkEvent::PeerJoined(peer) => {

                    print!("Peer: {} joined with public key: {:?}", peer.name, peer.identity)
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



            
        }
    }

    Ok(())
}