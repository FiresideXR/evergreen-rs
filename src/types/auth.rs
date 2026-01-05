// SPDX-License-Identifier: Apache-2.0
use std::str::FromStr;

type Timestamp = i64;


pub enum PassportError {

}

/// A Passport is a JWT that 
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Passport {
    jwt: String,
    provider: String, //com.firesidexr.auth
    peer_id: String,
    username: String,
    user_id: i64,
    flags: Option<Vec<String>>,
    expirey: Timestamp,
}

impl Passport {
    pub fn new_from_jwt(jwt: impl Into<String>) -> Self {

        let jwt: String = jwt.into();


        







        todo!()
    }

    pub fn to_jwt(&self) -> &str {
        &self.jwt
    }


    pub fn provider(&self) -> &String {
        return &self.provider
    }

    pub fn username(&self) -> &String {
        return &self.username
    }

    pub fn peer_id_str(&self) -> String {
        return self.peer_id.clone()
    }

    pub fn peer_id(&self) -> libp2p::identity::PeerId {
        return libp2p::identity::PeerId::from_str(&self.peer_id).expect("Something went wrong");
    }

    pub fn user_id(&self) -> i64 {
        return self.user_id
    }

    pub fn flags(&self) -> Option<&Vec<String>> {
        return self.flags.as_ref()
    }

    pub fn expirey(&self) -> chrono::DateTime<chrono::Utc> {
        return chrono::DateTime::from_timestamp(self.expirey, 0).unwrap()
    }
}


pub struct Provider {
    pub provider: String, //com.firesidexr.client
    pub public_keys: Vec<libp2p::identity::PublicKey>,
    //pub revoked_jwts: Vec<i64>,
}

pub struct ProviderList {
    _list: Vec<Provider>
}


impl ProviderList {

    pub fn create_passport(&self, _jwt: String) -> Result<Passport, PassportError> {

        todo!()
    }

}