// SPDX-License-Identifier: Apache-2.0
//
//
//

use std::error::Error;

use evergreen::network;
use libp2p::{identity::Keypair, Multiaddr};



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{

    let addr: Multiaddr = std::env::args().nth(0).unwrap().parse()?;

    let keypair = Keypair::generate_ed25519();

    let (mut server, mut server_handle) = network::server::Network::new_client(keypair, addr)
        .expect("Could not create clients");


    let handle = tokio::task::spawn(async move {server.run().await; });





    loop {
        if handle.is_finished() { return Ok(()); }

        tokio::select! {
            event = server_handle.get_event() => {
                match event {
                    Some(event) => println!("{event:?}"),
                    None => { break; }
                }
                
            }
        }
    }

    let _ = handle.await?;

    Ok(())
}