#[derive(Debug)]
pub enum Error {
    None,
    Multiaddr(libp2p::multiaddr::Error),
    Transport(libp2p::TransportError<std::io::Error>),
    Subscroption(libp2p::gossipsub::SubscriptionError),
    Dial(libp2p::swarm::DialError),
    Publish(libp2p::gossipsub::PublishError)
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::None => write!(f, "No Error"),
            Error::Multiaddr(error) => write!(f, "{error}"),
            Error::Transport(transport_error) => write!(f, "{transport_error}"),
            Error::Subscroption(subscription_error) => write!(f, "{subscription_error}"),
            Error::Dial(dial_error) => write!(f, "{dial_error}"),
            Error::Publish(publish_error) => write!(f, "{publish_error}")
        }
        
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(_: std::convert::Infallible) -> Self {
        Self::None
    }
}

impl From<libp2p::multiaddr::Error> for Error {
    fn from(value: libp2p::multiaddr::Error) -> Self {
        Self::Multiaddr(value)
    }
}

impl From<libp2p::TransportError<std::io::Error>> for Error {
    fn from(value: libp2p::TransportError<std::io::Error>) -> Self {
        Self::Transport(value)
    }
}

impl From<libp2p::gossipsub::SubscriptionError> for Error {
    fn from(value: libp2p::gossipsub::SubscriptionError) -> Self {
        Self::Subscroption(value)
    }
}

impl From<libp2p::swarm::DialError> for Error {
    fn from(value: libp2p::swarm::DialError) -> Self {
        Self::Dial(value)
    }
}

impl From<libp2p::gossipsub::PublishError> for Error {
    fn from(value: libp2p::gossipsub::PublishError) -> Self {
        Self::Publish(value)
    }
}