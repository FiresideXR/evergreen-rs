use crate::types::Identity;



pub enum RoomTransition {
    /// Disconnect from the current room, then connect to the next room. (Most standard model)
    Load,
    /// Wait to disconnect from previous room until fully connected to the next room. Transition to next room can then be instant.
    Instant,
    /// Connect to multiple rooms at the same time. Broadcast your packets to all rooms at once. 
    Simultaneous,
}



pub struct ClientConfig {

    pub identity: Identity,

    pub transition: RoomTransition,

    

}


impl ClientConfig {
    pub fn new(identity: Identity, transition: RoomTransition) -> Self {
        Self {
            identity, transition
        }
    }

    pub fn default(identity: Identity) -> Self {
        Self {
            identity,
            transition: RoomTransition::Load,
        }
    }

    pub fn with_transition(mut self, transition: RoomTransition) -> Self {
        self.transition = transition;
        self
    }
}