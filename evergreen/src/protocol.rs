


#[derive(Debug, Clone)]
pub struct Evergreen {

}



impl iroh::protocol::ProtocolHandler for Evergreen {
    fn accept(&self, connection: iroh::endpoint::Connection) -> impl Future<Output = Result<(), iroh::protocol::AcceptError>> + Send {
        todo!()
    }
}