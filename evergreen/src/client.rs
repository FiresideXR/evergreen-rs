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


use std::{collections::HashMap, hash::Hash};

use futures::FutureExt;
use iroh::{Endpoint, SecretKey, endpoint::{self, Connection, ConnectionError, RecvStream, presets}};

const ALPN: &str = "evergreen/0.1.0";

impl Client {
    pub async fn new(alpn: Vec<u8>, identity: SecretKey) -> Result<Self, iroh::endpoint::BindError> {

        let endpoint = Endpoint::builder(presets::N0)
            .alpns(vec![ALPN.into(), alpn])
            .secret_key(identity)
            .bind().await?;


        // Communication is key in any good relationship
        let (request_sender, request_reciever) = mpsc::channel::<()>(128);
        let (response_sender, response_reciever) = mpsc::channel::<()>(128);

        // Safeword
        let token = tokio_util::sync::CancellationToken::new();

        let task_handle = tokio::task::spawn(
            event_loop(endpoint.clone(), token.clone(), request_reciever, response_sender)
        );

        let wrapper = TaskWrapper{
            cancel_token: token,
            task_handle, 
            incoming_queue: response_reciever, 
            outoging_queue: request_sender
        };


        Ok(Client{
            endpoint,
            task: wrapper,
        })
    }
}


pub async fn stop_client(client: Client) {
    client.task.cancel();
}

pub struct Client {
    endpoint: Endpoint,
    task: TaskWrapper<(), (), ()>
}

use tokio::{pin, sync::mpsc};
use tokio_util::sync::CancellationToken;

impl Client {
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        self.task.is_done()
    }
}


async fn event_loop(
    endpoint: iroh::Endpoint,
    cancel_token: tokio_util::sync::CancellationToken, 
    mut incoming: mpsc::Receiver<()>, 
    outgoing: mpsc::Sender<()>,

){
    let mut connections: ConnectionList = ConnectionList { list: vec![] };

    loop {
        tokio::select! {
            // Shut it down!!!
            _ = cancel_token.cancelled() => {
                break;
            },
            new_command = incoming.recv() => { 
                let Some(command) = new_command else {break};

                todo!()
            },
            connection_event = &mut connections => {
                match connection_event {
                    Ok(stream) => todo!(),
                    Err(error) => todo!(),
                }
            }
        }
    }
}





struct ConnectionList {
    list: Vec<Connection>,
}

use std::task::Poll;

impl Future for ConnectionList {
    type Output = Result<endpoint::RecvStream, endpoint::ConnectionError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        
        for connection in &self.list {
            let future = connection.accept_uni();
            pin!(future);

            if let Poll::Ready(val) = future.poll(cx) {
                return Poll::Ready(val);
            };
        }
        
        Poll::Pending
    }
}

/// Wraps a [tokio::task] and pair of [mpsc::Sender] and [mpsc::Receiver]
/// 
/// For internal use only :P
struct TaskWrapper<A, B, C> {
    cancel_token: CancellationToken,
    task_handle: tokio::task::JoinHandle<A>,
    incoming_queue: tokio::sync::mpsc::Receiver<B>,
    outoging_queue: tokio::sync::mpsc::Sender<C>,
}

impl <A, B, C> TaskWrapper<A, B, C> {

    #[inline(always)]
    fn is_done(&self) -> bool {
        self.task_handle.is_finished() || self.incoming_queue.is_closed() || self.outoging_queue.is_closed()
    }

    #[inline(always)]
    async fn send(&self, value: C) -> Result<(), tokio::sync::mpsc::error::SendError<C>> {
        self.outoging_queue.send(value).await
    }

    #[inline(always)]
    async fn recv(&mut self) -> Option<B> {
        self.incoming_queue.recv().await
    }

    #[inline(always)]
    fn cancel(&self) {
        self.cancel_token.cancel();
    }
}

