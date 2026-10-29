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


const SERVER_LIST: [&str; 1] = ["localhost"];


#[tokio::main]
async fn main() -> anyhow::Result<()>{

    // Set up our client
    let identity = Identity::generate();
    let client_config = ClientConfig::default(identity).with_transition(Transition::LOAD);
    let client_handle = Client::create_and_start(client_config).await?;


    let server_list = ["localhost"];


    let room = find_room(server_list, "com.dungenrobot.chatroom");

    let client_list = discover_peers(server_list, room);

    client_handle.connect_to_room(room, server_list);

    post_status(server_list, room, identity);

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


                //???
                NetworkEvent::FriendshipRequested(peer) => {

                },
                NetworkEvent::FriendshipAccepted(peer) => {

                },
            }



            print!("{:?}", network_event)
        }
    }

    Ok(())
}