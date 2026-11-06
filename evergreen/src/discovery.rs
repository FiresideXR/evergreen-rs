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

use std::marker::PhantomData;

use reqwest::{IntoUrl, Response};

use crate::rooms::Room;

use crate::types::Identity;


#[derive(thiserror::Error, Debug)]
pub enum DiscoveryError {
    #[error("No urls were provided")]
    ListEmpty,

    #[error("A TLS backend could not be initialized or all attempts to contact urls failed or contained malformed responses.")]
    NetworkFailure,

    #[error("No rooms were discovered, but at least one of the urls was able to be contacted successfully.")]
    NoRoomsExist,

    #[error("Attempts to host a room failed")]
    RoomCreationRefused,
}


/// 
/// 
/// This was designed to be able to take in anything from `[String]` to `Vec<&str>` so the generics 
/// and lifetimes look softly like a nightmare
pub struct ServerList<I>(pub I)
where for<'a> &'a I: IntoIterator<Item: AsRef<str>>;



impl <I> ServerList<I> 
where for<'a> &'a I: IntoIterator<Item: AsRef<str>>
{

    /// Find all rooms matching the provided app_id and (optional) payload.
    pub async fn find_rooms(&self, app_id: &str, payload: Option<&str>) -> Result<Vec<Room>, DiscoveryError> {

        let client = new_client()?;

        for url in &self.0 {

            let request = client.get(url.as_ref()).header("app_id", app_id);

            if let Ok(response) = request.send().await {
                todo!("HTTP is unimplemented")
            }
        }


        return Err(DiscoveryError::NoRoomsExist)
    }

    /// Like ['find_room'](Self::find_rooms) but only returns the first result. 
    /// 
    /// This can run faster because it is 
    pub async fn find_one_room(&self, app_id: &str, payload: Option<&str>) -> Result<Room, DiscoveryError> {

        let client = new_client()?;

        for url in &self.0 {

            let request = client.get(url.as_ref()).header("app_id", app_id);

            if let Ok(response) = request.send().await {
                todo!("HTTP is unimplemented")
            }
        }


        return Err(DiscoveryError::NoRoomsExist)

    }

    /// A convenience method to call [`find_one_room`](Self::find_one_room) followed by
    /// 
    /// This does not allow you to set a payload or set salt for the created room. 
    /// It is assumed that if you are using a method like this, you kinda don't care anyway
    pub async fn find_or_host(&self, app_id: &str) -> Result<Room, DiscoveryError> {

        match self.find_one_room(app_id, None).await {
            Ok(room) => Ok(room),
            Err(_) => self.host(Room::new(app_id.to_string(), "".to_string())).await,
        }
    }

    pub async fn host(&self, room: Room) -> Result<Room, DiscoveryError> {
        let client = new_client()?;

        for url in &self.0 {

            let request = client.post(url.as_ref())
                .header("app_id", room.app_id())
                .header("payload", room.payload())
                .header("salt", format!("{:#X}", room.room_id()));

            if let Ok(response) = request.send().await {
                todo!("HTTP is unimplemented")
            };
        }


        todo!()
    }


    pub async fn find_peers(&self, room_id: u128) -> Result<Vec<iroh::EndpointAddr>, DiscoveryError> {
        todo!()
    }

    // if this fails, is it even our problem?
    pub async fn post_status(&self, identity: &Identity, room_id: u128) {

    }

    /// Pings all urls.
    /// 
    /// Returns either an Ok or an Err with how many servers refused to respond.
    pub async fn health_check(&self) -> Result<(), u16> {



        todo!()
    }
}


/// I got tired of writing this and it's probably good to make changed to the default client in a single place
fn new_client() -> Result<reqwest::Client, DiscoveryError> {
    reqwest::Client::builder().build().or(Result::Err(DiscoveryError::NetworkFailure))
}