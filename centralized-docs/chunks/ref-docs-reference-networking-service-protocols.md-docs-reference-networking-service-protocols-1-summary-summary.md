---
doc_id: ref/docs-reference-networking-service-protocols.md/docs-reference-networking-service-protocols
chunk_id: ref/docs-reference-networking-service-protocols.md/docs-reference-networking-service-protocols#1-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 80
summary: ### `SCTP` FEATURE STATE: `Kubernetes v1.20 [stable]` When using a network plugin that supports SCTP traffic, you can use SCTP for most Services. For `type: LoadBalancer` Services, SCTP support...
---

### `SCTP`
FEATURE STATE:
`Kubernetes v1.20 [stable]`
When using a network plugin that supports SCTP traffic, you can use SCTP for
most Services. For `type: LoadBalancer` Services, SCTP support depends on the cloud
provider offering this facility. (Most do not).
SCTP is not supported on nodes that run Windows.