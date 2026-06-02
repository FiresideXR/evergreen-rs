// Copyright (c) 2025 Anders Olsen
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

#[derive(Debug)]
pub enum Error {
    None,
    Multiaddr(libp2p::multiaddr::Error),
    Transport(libp2p::TransportError<std::io::Error>),
    //Subscroption(libp2p::gossipsub::SubscriptionError),
    Dial(libp2p::swarm::DialError),
    //Publish(libp2p::gossipsub::PublishError),
    Signing(libp2p::identity::SigningError),
    Utf8(std::string::FromUtf8Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::None => write!(f, "Infallible"),
            Error::Multiaddr(error) => write!(f, "{error}"),
            Error::Transport(transport_error) => write!(f, "{transport_error}"),
            //Error::Subscroption(subscription_error) => write!(f, "{subscription_error}"),
            Error::Dial(dial_error) => write!(f, "{dial_error}"),
            //Error::Publish(publish_error) => write!(f, "{publish_error}"),
            Error::Signing(signing_error) => write!{f, "{signing_error}"},
            Error::Utf8(utf8_error) => write!(f, "{utf8_error}"),
        }
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::Utf8(value)
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

// impl From<libp2p::gossipsub::SubscriptionError> for Error {
//     fn from(value: libp2p::gossipsub::SubscriptionError) -> Self {
//         Self::Subscroption(value)
//     }
// }

impl From<libp2p::swarm::DialError> for Error {
    fn from(value: libp2p::swarm::DialError) -> Self {
        Self::Dial(value)
    }
}

// impl From<libp2p::gossipsub::PublishError> for Error {
//     fn from(value: libp2p::gossipsub::PublishError) -> Self {
//         Self::Publish(value)
//     }
// }


impl From<libp2p::identity::SigningError> for Error {
    fn from(value: libp2p::identity::SigningError) -> Self {
        Self::Signing(value)
    }
}