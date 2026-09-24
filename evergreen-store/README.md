


# Storage

## The Problem

A key issue with decentralization is the problem of persistence. Data can be checked, updated, and synchronized while peers are connected to each other, but this does not persist between sessions. In some cases this is fine. If a user changes their outfit while another is offline, this only needs to be synchronized once the two users reconnect in a future session. But if we are developing an application with a dynamic and persistent shared space, this becomes a problem. 

## One Solution (+ Example)

It is important to consider whether fully synchronized state is actually necessary and/or desirable. There exist many protocols for reconciling and merging changes in offline work. See [Automerge](https://automerge.org/). Many game stores also provide cloud saves, meaning a user's local changes are *already* backed up in a secure server. For many designs this may be enough. 

## Our Solution
A contract is written for the storage object. This contract defines the type of storage needed, who is allowed to modify it and the contents of the store. 

> ![NOTE]
> To make a store "private", the contract marks the store as only accessible to users marked with explicit read permissions. This is a hint flag for the storage node but could be ignored. Encrypt sensitive data where required.

A normal user can update state within this storage 

## Advantages

Mutable data attached to a room has many advantages. For our event services, we can store the description, tags, and other information inside of a storage object that directs to a room id. If a typo is made or details around the event need to be updated, this can be done. Rather than via the contract system, where mutating the data requires updating the room id and moving all users (which is a valid migration system in certain cases). 


## Important Notes

It is important to remember that like data send between clients, peers are ultimately the ones verifying changes to storage blobs. This means your game <ins>will</ins> be vulnerable to certain exploits. 

Storage nodes may have strict limits on the size of the blob. Consider wether a full history of all changes is needed, or whether an admin could squash the history and prune violating branches. Ensure that all data written to a blob is data that *needs* to be written to the blob. Some data about a player may be best stored locally. 

Storage nodes can delete blobs at any time, or even just delete parts of them, typically if it has been a long time since it was last accessed. An application may wish to maintain backups of their blobs across multiple storage nodes or locally on their client.

Storage blobs are not truly private, the private flag is a suggestion to the node that it may choose to honor or not.

Blob admins (defined at creation) cannot be reassigned or removed. These are the only users permitted to delete history (usually to squash changes). 


# Trust and Privacy



It is important to understand the types of attacks that 





```
timestamp: u64
last_block: u256
data: [u8]
signature: [u8]
```