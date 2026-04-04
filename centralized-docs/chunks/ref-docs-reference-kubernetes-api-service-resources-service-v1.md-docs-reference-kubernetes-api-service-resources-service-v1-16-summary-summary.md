---
doc_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1
chunk_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1#16-summary
chunk_level: summary
chunk_type: prose
heading: ServiceSpec
token_count: 66
summary: * **ports.protocol** (string) The IP protocol for this port. Supports \"TCP\", \"UDP\", and \"SCTP\". Default is TCP. Possible enum values: * `\"SCTP\"` is the SCTP protocol. * `\"TCP\"` is the TCP protocol. *...
---

* **ports.protocol** (string)
The IP protocol for this port. Supports "TCP", "UDP", and "SCTP". Default is TCP.
Possible enum values:
* `"SCTP"` is the SCTP protocol.
* `"TCP"` is the TCP protocol.
* `"UDP"` is the UDP protocol.