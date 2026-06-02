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

use std::str::FromStr;

use anyhow::Context;
use iroh::{Endpoint, EndpointAddr, PublicKey, endpoint::{Connection, presets}};



const ALPN: &'static [u8; 15] = b"evergreen/0.1.0";


#[tokio::main]
async fn main() -> anyhow::Result<()>{

    let endpoint = Endpoint::builder(presets::N0).alpns(vec![ALPN.to_vec()]).bind().await?;

    endpoint.online().await;

    println!("{}", endpoint.id());

    match std::env::args().len() {
        1 => {
            init_recv(endpoint).await
        },
        2 => {

            let public_key_string = &std::env::args().collect::<Vec<String>>()[1];

            let public_key = PublicKey::from_str(public_key_string)?;

            //public_key.is_v

            //println!()

            let addr = EndpointAddr::new(public_key);

            init_send(endpoint, addr).await
        }
        _ => {
            println!("D:");
            Ok(())
        }
    }
}



async fn init_send(endpoint: Endpoint, addr: EndpointAddr) -> anyhow::Result<()> {

    let connection = endpoint.connect(addr, ALPN).await?;

    let mut send_stream = connection.open_uni().await?;

    send_stream.write_all(b"fuck around and find out").await?;

    send_stream.finish()?;

    let connection_error = connection.closed().await;

    println!("{connection_error}");

    endpoint.close().await;

    Ok(())
}


async fn init_recv(endpoint: Endpoint) -> anyhow::Result<()> {

    let connection = endpoint.accept().await.context("Endpoint closed")?.await?;

    println!("Accepted connection");

    let mut recv_stream = connection.accept_uni().await?;

    println!("Accepted stream.");

    let bytes = recv_stream.read_to_end(300).await?;

    println!("{}", String::from_utf8(bytes)?);

    connection.close(1u8.into(), b"done");

    endpoint.close().await;

    Ok(())
}