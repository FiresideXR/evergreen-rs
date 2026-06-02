# Evergreen

Evergreen is a networking backend developed for use in FiresideXR, built on top of the [libp2p](https://libp2p.io/).

## Security

If you believe to have discovered a security issue, reach out over email.

## Packets

The data a packet carries is a key/value pair of a UTF-8 string and a byte array.
Packets also contain a sequence number, the identity of the sending party, and an optional signature.

All future versions of Evergreen clients and servers should support a basic set of packets.

The basic kinds of packets are as follows:

- Message
- Voice
- Movement
- Add Passport
- Set Avatar


"Message" is a simple packet that contains a UTF-8 encoded string in its payload. 
It exists for basic testing and should not be used in production environments.

Packets are converted into typed enums by this library before your client or server recieves them in their run function.

I want to do this with protobufs at some point but I need to get everything working first :P

## Passports

An identity in libp2p is a public/private keypair. For some users and use cases this is sufficient identity. For FiresideXR we have additional constraints that require we have some way to link these keypairs to a user account. To solve this, we've developed the concept of passports.

A passport is a [JWT](https://www.jwt.io/introduction) that links some information to a player's public key. 
For Evergreen, this is a userid, username, and a set of flags. These expire after 30 minutes, so they also act as a form of moderation. A player can be banned by simply not reissuing them a passport. 

Passports are signed and issued by account providers. For FiresideXR we only have a single provider that all clients and servers recognize. But in another use case, servers and clients could choose to accept passports from multiple providers.

### Flags

Passports include a set of hashed flags. A player can choose to share the un-hashed flags with another user to show account age, moderation permissions, a developer tag, or proof of a subscription. These are pieces of information that a user can choose to share or not share.

## Networks

There are two kinds of networks that a peer might connect to. 

An unimplemented network is discussed as well.

### P2P Mesh

A p2p mesh network involves every peer having a direct connection to every other peer. This connection is encrypted with the peer id, so packets can be set without needing another layer of verification. A packet from that peer is verifiably from that peer.
 
This kind of network usually can only handle single rooms, and small numbers of peers.

### Hub

A hub server is a more traditional game server. They are also responsible for hole-punching peers in order to establish p2p mesh networks.

Some hubs may be considered "trusted" by clients. 

Nodes are responsible for a variety of tasks. 


### (Unimplemented) P2P Host

This network would be more like a traditional p2p game network, where one peer hosts and handles the session for all connecting peers.
At the moment implementation of this network wouldn't provide many benefits over simply hosting an untrusted network.


## Hub Settings

Hubs can be to three security levels.

### Force Untrusted - 0

This forces all users to sign all packets. Unsigned packets are discarded. 

> [!NOTE]
> This does not *verify* packets, only ensure that there is *some* data in the signature field.

### Allow Trusted - 10

Peers may send a signed "trust packet" that communicates to all users that it trusts the hub server and that it may choose not to provide a signature with some or all of its packets. This trust must be re-communicated on a regular interval.

### Force Trusted - 20

Packets are stripped of their signatures by the

> [!WARNING]
> This behavior is not recommeneded for the majority of use cases. 

### Force Trusted - 30

Packets are stripped of their signatures. This forces peers to either trust the hub or disconnect.


# Safety

