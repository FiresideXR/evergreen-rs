
use iroh::{Endpoint, protocol::{RouterBuilder, Router}};
use crate::protocol::{Evergreen, ALPN};





pub struct Client {
    evergreen: Evergreen,
    router: Router,
}

use crate::types::Identity;

impl Client {

    /// Creates a [Client]. This starts a few things in motion at once.
    /// 
    /// 1. Creates an [Iroh][iroh] endpoint. This kickstarts discoverability via DNS and begins connections to relays
    /// 
    /// 2. Creates a [Evergreen Protocol Instance][Evergreen]
    /// 
    /// Once this functions returns, the endpoint is "online" and has connected to a relay.
    /// 
    /// This function waits for [Endpoint::online()] to return. 
    /// Please refer to Iroh documentation regarding setting timeouts, as this function has none implemented by default.
    /// 
    /// This function takes an optional callback for updates about the client. 
    /// See [UpdateCallback] for more information.
    /// 
    /// As you can see this abstracts over much of the inner workings of Evergreen and how it interfaces with other libraries like [iroh].
    /// Functions and types are provided via [protocol][crate::protocol], [wire][crate::wire], and [types][crate::types] 
    /// that provide more granular control over your network connections and data.
    pub async fn new(identity: Identity) -> Result<Self, iroh::endpoint::BindError> {

        let endpoint = Endpoint::builder(iroh::endpoint::presets::N0)
            .alpns(vec![ALPN.into()])
            .secret_key(identity)
            .bind().await?;

        let (evergreen, handler) = crate::protocol::new(endpoint.clone());

        let router = RouterBuilder::new(endpoint.clone())
            .accept(crate::protocol::ALPN, handler)
            .spawn();

        // Make sure we're actually online
        endpoint.online().await;

        Ok(Client{
            evergreen,
            router,
        })
    }

    /// This functions takes ownership of a [Client] and then kills it. 
    pub async fn stop_client(self) {
        let _ = self.router.shutdown().await;
    }
}