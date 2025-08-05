use std::str::FromStr;

pub enum PassportError {

}


#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Passport {
    provider: String,
    peer_id: String,
    username: String,
    user_id: i64,
    flags: Option<Vec<String>>,
    expirey: i64,
}

impl Passport {
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