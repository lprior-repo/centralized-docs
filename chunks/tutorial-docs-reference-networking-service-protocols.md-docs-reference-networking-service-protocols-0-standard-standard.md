---
doc_id: tutorial/docs-reference-networking-service-protocols.md/docs-reference-networking-service-protocols
chunk_id: tutorial/docs-reference-networking-service-protocols.md/docs-reference-networking-service-protocols#0-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 281
summary: ## Table of Contents      - [`SCTP`](#sctp)       - [Support for multihomed SCTP associations](#support-for-multihomed-sctp-associations)     - [`TCP`](#tcp)     - [`UDP`](#udp)     - [HTTP](#http)  ...
---

## Table of Contents

    - [`SCTP`](#sctp)
      - [Support for multihomed SCTP associations](#support-for-multihomed-sctp-associations)
    - [`TCP`](#tcp)
    - [`UDP`](#udp)
    - [HTTP](#http)
      - [Note:](#note)
    - [TLS](#tls)
  - [Feedback](#feedback)

---

### `SCTP`
FEATURE STATE:
`Kubernetes v1.20 [stable]`
When using a network plugin that supports SCTP traffic, you can use SCTP for
most Services. For `type: LoadBalancer` Services, SCTP support depends on the cloud
provider offering this facility. (Most do not).
SCTP is not supported on nodes that run Windows.
#### Support for multihomed SCTP associations
The support of multihomed SCTP associations requires that the CNI plugin can support the assignment of multiple interfaces and IP addresses to a Pod.
NAT for multihomed SCTP associations requires special logic in the corresponding kernel modules.
### `TCP`
You can use TCP for any kind of Service, and it's the default network protocol.
### `UDP`
You can use UDP for most Services. For `type: LoadBalancer` Services,
UDP support depends on the cloud provider offering this facility.