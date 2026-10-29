use sha2::Digest;




struct Room {
    app_id: String,
    payload: String,

    hashed_room_id: u128,
}


impl Room {

    fn new(app_id: String, payload: String) -> Self {

        let mut hasher = sha2::Sha224::new();
        hasher.update(&app_id);
        hasher.update(&payload);
        let hash = hasher.finalize();

        let room_id = u128::from_be_bytes(hash[..16].try_into().unwrap()); //danger danger danger

        Self { 
            app_id, 
            payload,
            hashed_room_id: room_id,
        }
    }

    fn app_id(&self) -> &String {
        &self.app_id
    }
    fn room_id(&self) -> u128 {
        self.hashed_room_id
    }
}
