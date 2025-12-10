// SPDX-License-Identifier: Apache-2.0
//
//
//

use std::error::Error;

use firesidexr_evergreen as evergreen;

use evergreen::network;
use evergreen::types::*;
use libp2p::{identity::Keypair, Multiaddr};
use tokio::io::AsyncBufReadExt;



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{

    let str = std::env::args().nth(1).unwrap();

    println!("{str}");

    let addr: Multiaddr = str.parse()?;

    let keypair = Keypair::generate_ed25519();

    let (mut server, mut server_handle) = network::untrusted::Network::new_client(keypair, addr)
        .expect("Could not create clients");


    let handle = tokio::task::spawn(async move {server.run().await; });

    //let chat = IdentTopic::new("chat");

    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();

    loop {
        if handle.is_finished() { return Ok(()); }

        tokio::select! {

            Ok(Some(line)) = stdin.next_line() => {
                server_handle.send_data(PacketData::Message(line)).await
            }

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