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

use std::error::Error;

use libp2p::identity::Keypair;
use firesidexr_evergreen::server;
use tracing_subscriber::EnvFilter;


use firesidexr_evergreen::types::*;





use libp2p::multiaddr;
use std::net::Ipv6Addr;



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{

    let _ = tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).try_init();

    let keypair = Keypair::generate_ed25519();
    let addr = libp2p::Multiaddr::empty().with(multiaddr::Protocol::Ip6(Ipv6Addr::UNSPECIFIED)).with(multiaddr::Protocol::QuicV1);
    //let addr: libp2p::Multiaddr = "/ip6/::/udp/0/quic-v1".parse()?;

    let (mut server, mut server_handle) = 
        server::untrusted::Network::new_server(keypair, addr).expect("Could not create server");

    let handle = tokio::task::spawn(async move { server.run().await; });

    println!("Server started");

    loop {
        println!("Loop");

        if handle.is_finished() { break }

        tokio::select! {
            Some(resp) = server_handle.get_event() => match resp {
                Response::Network(NetworkUpdate::AliveWithAddr(addr)) => {
                    println!("Alive with addr: {addr}")
                },

                Response::Network(NetworkUpdate::Disconnected) => {
                    println!("Disconnected");
                    break;
                },

                Response::Client(resp) => {
                    let peer = resp.peer;
                    let data = resp.data;
                    println!("{peer}, {data:?}");
                },

                _ => (),
            }
        }
    }


    let _ = handle.await?;

    Ok(())
}