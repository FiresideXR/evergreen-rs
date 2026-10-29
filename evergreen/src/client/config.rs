use crate::types::Identity;



pub enum RoomTransition {
    /// Disconnect from the current room, then connect to the next room. (Most standard model)
    LoadingScreen,
    /// Wait to disconnect from previous room until fully connected to the next room. Transition to next room can then be instant.
    Instant,
    /// Connect to multiple rooms at the same time. Broadcast your packets to all rooms at once. 
    Simultaneous,
}



pub struct ClientConfig {

    pub(crate) identity: Identity,

    pub(crate) transition: RoomTransition,



}


impl ClientConfig {
    pub fn default(identity: Identity) -> Self {
        Self {
            identity,
            transition: RoomTransition::LoadingScreen,
        }
    }
}