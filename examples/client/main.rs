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

use firesidexr_evergreen as evergreen;

use evergreen::server;
use evergreen::types::*;
use libp2p::{identity::Keypair, Multiaddr};
use tokio::io::AsyncBufReadExt;



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{

    let str = std::env::args().nth(1).unwrap();

    println!("{str}");

    let addr: Multiaddr = str.parse()?;

    let keypair = Keypair::generate_ed25519();

    let (mut server, mut server_handle) = server::untrusted::Network::new_client(keypair, addr)
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