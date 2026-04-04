---
doc_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1
chunk_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1#17-summary
chunk_level: summary
chunk_type: prose
heading: ServiceSpec
token_count: 99
summary: * `\"SCTP\"` is the SCTP protocol. * `\"TCP\"` is the TCP protocol. * `\"UDP\"` is the UDP protocol. * **ports.name** (string) The name of this port within the service. This must be a DNS\_LABEL. All ports...
---

* `"SCTP"` is the SCTP protocol.
* `"TCP"` is the TCP protocol.
* `"UDP"` is the UDP protocol.
* **ports.name** (string)
The name of this port within the service. This must be a DNS\_LABEL. All ports within a ServiceSpec must have unique names. When considering the endpoints for a Service, this must match the 'name' field in the EndpointPort. Optional if only one ServicePort is defined on this service.