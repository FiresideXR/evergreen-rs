# Evergreen

Evergreen is a networking backend developed for use in FiresideXR.

This library for Evergreen is licensed under Apache-2.0.


The primary attacks mitigated against are ones where a peer or untrusted network attempts to impersonate another peer.
Since this protocol is designed for social apps, this is the major concern.



The basic kinds of packets are as follows:

- Message
- Voice
- Movement
- Add Passport
- Set Avatar


## Networks

There are three kinds of networks that a peer might connect to. 

Two extra unimplemented networks are discussed as well.

### P2P Mesh

A p2p mesh network involves every peer having a direct connection to every other peer. This connection is encrypted with the peer id, so packets can be set without needing another layer of verification. A packet from that peer is verifiably from that peer.
 
This kind of network usually can only handle single rooms, and small numbers of peers.

### Untrusted

An untrusted network is hub-spoke and not trusted by its peers. This means all packets must be signed and verified.
Packets are also sent with incrementing IDs to protect against replay attacks.

### Trusted

A trusted network is hub-spoke and trusted by all its peers. No packets are sent with signatures. This is the most like a traditional game server. 


### (Unimplemented) P2P Host

This network would be more like a traditional p2p game network, where one peer hosts and handles the session for all connecting peers.
At the moment implementation of this network wouldn't provide many benefits over simply hosting an untrusted network.

### (Unimplemented) Mixed Trust

This type of network would allow for peers to either operate in a trusted or untrusted mode. A peer would choose to sign all packets or instead provide a signed message to all peers confirming that it trusts the network and no signatures will be provided for its packets. 

Currently unimplemented for scope reasons. 

This network is expected to surpass the Untrusted type.