


pub enum FilterResult {
    AcceptConnection,
    CloseConnection,
}

// You have no idea how much time I spent trying to make this type any cleaner than iroh::protocol::IncomingFilter and just totally failed
// 
// In an ideal world I'd have this optimized into the handler itself but at this point I truly have stopped caring
// Who care about v-tables. This function gets called once per connection. Literally doesn't matter.
/// I hate this type so much more than you think I possibly could
pub type ConnectionFilter = std::sync::Arc<dyn Fn(&iroh::EndpointId, &crate::wire::InitialPayload) -> FilterResult + Send + Sync>;



pub(crate) fn default_filter(addr: &iroh::EndpointId, info: &crate::wire::InitialPayload) -> FilterResult {

    todo!()
}