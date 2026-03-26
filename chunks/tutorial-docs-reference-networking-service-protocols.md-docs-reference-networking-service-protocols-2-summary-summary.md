---
doc_id: tutorial/docs-reference-networking-service-protocols.md/docs-reference-networking-service-protocols
chunk_id: tutorial/docs-reference-networking-service-protocols.md/docs-reference-networking-service-protocols#2-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 111
summary: #### Support for multihomed SCTP associations The support of multihomed SCTP associations requires that the CNI plugin can support the assignment of multiple interfaces and IP addresses to a Pod. NAT...
---

#### Support for multihomed SCTP associations
The support of multihomed SCTP associations requires that the CNI plugin can support the assignment of multiple interfaces and IP addresses to a Pod.
NAT for multihomed SCTP associations requires special logic in the corresponding kernel modules.
### `TCP`
You can use TCP for any kind of Service, and it's the default network protocol.
### `UDP`
You can use UDP for most Services. For `type: LoadBalancer` Services,
UDP support depends on the cloud provider offering this facility.