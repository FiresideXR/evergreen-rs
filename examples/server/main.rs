use std::{error::Error, time::Duration};

use libp2p::identity::Keypair;
use evergreen::network::{self, server::NetworkUpdate};
use tracing_subscriber::EnvFilter;


use evergreen::types::*;




#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{

    let _ = tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).try_init();

    let keypair = Keypair::generate_ed25519();

    let (handle, sender, mut reciever) = 
        network::server::run(keypair.clone(), true, "/ip4/0.0.0.0/udp/0/quic-v1".parse()?);

    let _ = tokio::time::sleep(Duration::from_secs(10));

    let _ = sender.send(PacketData::Message("Hello!".into())).await;

    loop {

        if handle.is_finished() { break }


        match reciever.recv().await {
            Some(resp) => { match resp {
                network::server::Response::Network(NetworkUpdate::Disconnected) => {
                    break;
                },

                network::server::Response::Client(resp) => {
                    let peer = resp.peer;
                    let data = resp.data;
                    println!("{peer}, {data:?}");
                }

                _ => ()
            }},
            None => (),
        }
    }


    let _ = handle.await;

    Ok(())
}