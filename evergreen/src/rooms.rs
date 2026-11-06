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
