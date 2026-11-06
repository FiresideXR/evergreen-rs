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