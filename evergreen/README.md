# Evergreen

Evergreen is a networking backend developed for use in FiresideXR, built on top of the [iroh][https://iroh.computer/].

> ![WARNING]
> Evergreen is in early development and subject to sudden breaking changes. 

### Security

If you believe to have discovered a security issue, reach out over email.

## Overview

All sessions (rooms) in Evergreen are peer to peer mesh networks. Each peer maintains a connection to every other user in the session. 



## Passports

An identity in libp2p is a public/private keypair. For some users and use cases this is sufficient identity. For FiresideXR we have additional constraints that require we have some way to link these keypairs to a user account. To solve this, we've developed the concept of passports.

A passport is a [JWT](https://www.jwt.io/introduction) that links some information to a player's public key. 
For Evergreen, this is a userid, username, and a set of flags. These expire after 30 minutes, so they also act as a form of moderation. A player can be banned by simply not reissuing them a passport. 

Passports are signed and issued by account providers. For FiresideXR we only have a single provider that all clients and servers recognize. But in another use case, servers and clients could choose to accept passports from multiple providers.

### Flags

Flags are hidden fields in a passport that can be optionally shared by a client. They are stored in a passport as a set of hashed strings. A player can choose to share the un-hashed strings with another user to show account age, moderation permissions, a developer tag, or proof of a subscription. 

The applications for flags are numerous but should be considered carefully. Each flag takes up extra space and compute. Fields that can be provided outright should be. An ideal use case could be an over 18 flag. There are dangers to transparently seeing this flag (or lack thereof). 

# Safety

