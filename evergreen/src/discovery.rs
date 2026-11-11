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


pub trait ServerList: Sync + Send{
    /// Find all rooms matching the provided app_id and (optional) payload.
    fn find_rooms(&self, app_id: &str, payload: Option<&str>) -> impl std::future::Future<Output = Result<Vec<Room>, DiscoveryError>> + Send;

    /// Like ['find_room'](Self::find_rooms) but only returns the first result. 
    /// 
    /// This can run faster because it is 
    fn find_one_room(&self, app_id: &str, payload: Option<&str>) -> impl std::future::Future<Output = Result<Room, DiscoveryError>> + Send;

    /// A convenience method to call [`find_one_room`](Self::find_one_room) followed by
    /// 
    /// This does not allow you to set a payload or set salt for the created room. 
    /// It is assumed that if you are using a method like this, you kinda don't care anyway
    fn find_or_host(&self, app_id: &str) -> impl std::future::Future<Output = Result<Room, DiscoveryError>> + Send;

    fn host(&self, room: Room) -> impl std::future::Future<Output = Result<Room, DiscoveryError>> + Send;

    fn find_peers(&self, room_id: u128) -> impl std::future::Future<Output = Result<Vec<iroh::EndpointAddr>, DiscoveryError>> + Send;

    fn post_status(&self, identity: &Identity, room_id: u128) -> impl std::future::Future<Output = ()> + Send;

    fn health_check(&self) -> impl std::future::Future<Output = Result<(), u16>> + Send;
}

impl <I: Sync + Send> ServerList for I
where 
    for<'a> &'a I: IntoIterator<Item: AsRef<str> + Send>,

    // I don't know why I need this line but I'm gonna try to not worry about it
    for<'a> <&'a I as IntoIterator>::IntoIter: Send, 
{

    async fn find_rooms(&self, app_id: &str, _payload: Option<&str>) -> Result<Vec<Room>, DiscoveryError> {

        let client = new_client()?;

        for url in self {

            let request = client.get(url.as_ref()).header("app_id", app_id);

            if let Ok(_response) = request.send().await {
                todo!("HTTP is unimplemented")
            }
        }

        return Err(DiscoveryError::NoRoomsExist)
    }

    async fn find_one_room(&self, app_id: &str, _payload: Option<&str>) -> Result<Room, DiscoveryError> {

        let client = new_client()?;

        for url in self {

            let request = client.get(url.as_ref()).header("app_id", app_id);

            if let Ok(_response) = request.send().await {
                todo!("HTTP is unimplemented")
            }
        }


        return Err(DiscoveryError::NoRoomsExist)

    }

    
    async fn find_or_host(&self, app_id: &str) -> Result<Room, DiscoveryError> {

        match self.find_one_room(app_id, None).await {
            Ok(room) => Ok(room),
            Err(_) => self.host(Room::new(app_id.to_string(), "".to_string())).await,
        }
    }

    async fn host(&self, room: Room) -> Result<Room, DiscoveryError> {
        let client = new_client()?;

        for url in self {

            let request = client.post(url.as_ref())
                .header("app_id", room.app_id())
                .header("payload", room.payload())
                .header("salt", format!("{:#X}", room.room_id()));

            if let Ok(_response) = request.send().await {
                todo!("HTTP is unimplemented")
            };
        }


        todo!()
    }


    async fn find_peers(&self, _room_id: u128) -> Result<Vec<iroh::EndpointAddr>, DiscoveryError> {
        todo!()
    }

    // if this fails, is it even our problem?
    async fn post_status(&self, _identity: &Identity, _room_id: u128) {

    }

    /// Pings all urls.
    /// 
    /// Returns either an Ok or an Err with how many servers refused to respond.
    async fn health_check(&self) -> Result<(), u16> {



        todo!()
    }
}


/// I got tired of writing this and it's probably good to make changed to the default client in a single place
fn new_client() -> Result<reqwest::Client, DiscoveryError> {
    reqwest::Client::builder().build().or(Result::Err(DiscoveryError::NetworkFailure))
}