---
doc_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1
chunk_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1#69-summary
chunk_level: summary
chunk_type: prose
heading: ServiceStatus
token_count: 80
summary: * **loadBalancer.ingress.ports.protocol** (string), required Protocol is the protocol of the service port of which status is recorded here The supported values are: \"TCP\", \"UDP\", \"SCTP\" Possible enum...
---

* **loadBalancer.ingress.ports.protocol** (string), required
Protocol is the protocol of the service port of which status is recorded here The supported values are: "TCP", "UDP", "SCTP"
Possible enum values:
* `"SCTP"` is the SCTP protocol.
* `"TCP"` is the TCP protocol.
* `"UDP"` is the UDP protocol.