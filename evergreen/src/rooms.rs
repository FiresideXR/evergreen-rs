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

use sha2::Digest;


pub struct Room {
    app_id: String,
    payload: String,
    salt: u64,
    hashed_room_id: u128,
}


impl Room {

    #[inline(always)]
    pub fn new(app_id: String, payload: String) -> Self {
        Self::new_with_set_salt(app_id, payload, rand::random())
    }

    pub fn new_with_set_salt(app_id: String, payload: String, salt: u64) -> Self {

        let mut hasher = sha2::Sha224::new();
        hasher.update(&app_id);
        hasher.update(&payload);
        hasher.update(salt.to_be_bytes());
        let hash = hasher.finalize();

        let room_id = u128::from_be_bytes(hash[..16].try_into().unwrap()); //danger danger danger

        Self {
            app_id,
            payload,
            salt,
            hashed_room_id: room_id,
        }
    }

    pub fn app_id(&self) -> &String {
        &self.app_id
    }

    pub fn payload(&self) -> &String {
        &self.payload
    }

    pub fn room_id(&self) -> u128 {
        self.hashed_room_id
    }

    pub fn salt(&self) -> u64 {
        self.salt
    }
}
