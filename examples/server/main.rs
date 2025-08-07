// SPDX-License-Identifier: Apache-2.0

use std::error::Error;

use libp2p::identity::Keypair;
use firesidexr_evergreen::network;
use tracing_subscriber::EnvFilter;


use firesidexr_evergreen::types::*;









#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{

    let _ = tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).try_init();

    let keypair = Keypair::generate_ed25519();
    let addr: libp2p::Multiaddr = "/ip4/0.0.0.0/udp/0/quic-v1".parse()?;

    let (mut server, mut server_handle) = 
        network::server::Network::new_server(keypair, addr).expect("Could not create server");

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