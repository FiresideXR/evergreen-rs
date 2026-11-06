# Evergreen

Evergreen is a networking backend developed for use in FiresideXR, built on top of the [iroh][https://iroh.computer/].

> ![WARNING]
> Everything is broken and nothing works properly. 
<!-- > Evergreen is in early development and subject to sudden breaking changes.  -->


### Security

If you believe to have discovered a security issue, reach out via email.

## Overview

All sessions (rooms) in Evergreen are peer to peer mesh networks. Each peer maintains a connection to every other user in the session. 






## Why use Evergreen?

Iroh is a fantastic library for establishing peer to peer connections, but it lacks robust forms of discovery for *new* peers. It also makes no assumption about what Iroh will *actually be used for*. These are great assumptions for Iroh to make, but we have more specific requirements when developing games.

In Evergreen, a mesh network of peers that agree on the rules of a game and a shared state is a `Room`. A `Room` is defined by an `APP_ID`, static information about that room (the `PAYLOAD`), and a random `SALT`. The `ROOM_ID` is SHA256 hash of these values. It is a shorthand for referring to the room but is not a replacement for the other identifying information.

A `Room` is discovered by querying Evergreen nodes. These nodes are basic HTTP servers, designed for matchmaking and persistent data storage. A user can query these nodes for rooms that match the given `APP_ID` and optional `PAYLOAD`. This means that nodes are game-agnostic! They will 

Some games may choose to use an empty `PAYLOAD` or `SALT` for various reasons. The "Campfire" space in FiresideXR uses a payload of `CAMPFIRE` with a salt of `0`.





## Passports

An identity in iroh is a public/private keypair. For some users and use cases this is sufficient identity. For FiresideXR we have additional constraints that require we have some way to link these keypairs to a user account. To solve this, we've developed the concept of passports.

A passport is a [JWT](https://www.jwt.io/introduction) that links some information to a player's public key. 
For Evergreen, this is a userid, username, and a set of flags. These expire after 30 minutes, so they also act as a form of moderation. A player can be banned by simply not reissuing them a passport. 

Passports are signed and issued by account providers. For FiresideXR we only have a single provider that all clients and servers recognize. But in another use case, servers and clients could choose to accept passports from multiple providers.

<!-- ### Flags

Flags are hidden fields in a passport that can be optionally shared by a client. They are stored in a passport as a set of hashed strings. A player can choose to share the un-hashed strings with another user to show account age, moderation permissions, a developer tag, or proof of a subscription. 

The applications for flags are numerous but should be considered carefully. Each flag takes up extra space and compute. Fields that can be provided outright should be. An ideal use case could be an over 18 flag. There are dangers to transparently seeing this flag (or lack thereof).  -->

# Safety



# Supporting Evergreen

There are a few ways to help maintain development and stability of Evergreen. 

- Host an Iroh relay! 
- We're open source, please contribute!
- We also will be making donations available in the future. 
- Host a node! The more nodes on the network, the better Evergreen will be for everyone!
- Support Iroh! Development and maintenance of Iroh helps all projects downstream
