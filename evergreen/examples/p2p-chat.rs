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

use std::{io, str::FromStr, thread};

use anyhow::Context;
use iroh::{Endpoint, EndpointAddr, PublicKey, endpoint::{Connection, RecvStream, SendStream, presets}};
use tokio::{io::AsyncReadExt, sync::mpsc};


const ALPN: &[u8; 8] = b"p2p-chat";

#[tokio::main]
async fn main() -> anyhow::Result<()>{


    let endpoint = Endpoint::builder(presets::N0).alpns(vec![ALPN.to_vec()]).bind().await?;

    endpoint.online().await;

    println!("Online with id: {}", endpoint.id());


    let connection = init_connection(&endpoint).await?;

    let (send_io, mut recv_io) = mpsc::channel::<String>(10);

    
    let _ = thread::spawn(move || -> anyhow::Result<()> {
        let stdin = io::stdin();

        loop {
            let mut string_buffer: String = "".into();
            stdin.read_line(&mut string_buffer)?;
            string_buffer = string_buffer.trim().into();
            send_io.blocking_send(string_buffer)?;
        }
    });



    loop {

        let mut read_buffer = [0u8; 256];

        tokio::select! {
            connection_error = connection.closed() => {
                println!("{}", connection_error);
                break;
            },
            conn_uni = connection.accept_uni() => {
                match conn_uni {
                    Ok(mut recv_stream) => {

                        let len = recv_stream.read(&mut read_buffer).await?.unwrap();

                        let string = String::from_utf8(read_buffer[0..len].to_vec())?;

                        println!("> {}", string);
                    }
                    Err(_) => {
                        println!("Connection closed.");
                        break;
                    }
                }
            },
            io = recv_io.recv() => {
                match io {
                    Some(input_string) => {
                        let mut send_data = connection.open_uni().await?;
                        send_data.write(input_string.as_bytes()).await?;
                        send_data.finish()?;
                    }
                    None => {
                        println!("No more input to write. Closing connection.");
                        connection.close(1u8.into(), b"No more data to send");
                        break;
                    }
                }
            }


        }

    }



    endpoint.close().await;

    

    Ok(())
}




async fn init_connection(endpoint: &Endpoint) -> anyhow::Result<Connection> {
    if std::env::args().len() == 2 {

        let other_peer_id = &std::env::args().collect::<Vec<String>>()[1];

        let addr = EndpointAddr::new(PublicKey::from_str(other_peer_id)?);

        let connection= endpoint.connect(addr, ALPN).await?;

        Ok(connection)
    } else {
        let connection = endpoint.accept().await.context("Endpoint closed")?.await?;

        Ok(connection)
    }

}