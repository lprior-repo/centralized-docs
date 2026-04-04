---
doc_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1
chunk_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1#70-summary
chunk_level: summary
chunk_type: prose
heading: ServiceStatus
token_count: 128
summary: * `\"SCTP\"` is the SCTP protocol. * `\"TCP\"` is the TCP protocol. * `\"UDP\"` is the UDP protocol. * **loadBalancer.ingress.ports.error** (string) Error is to record the problem with the service port The...
---

* `"SCTP"` is the SCTP protocol.
* `"TCP"` is the TCP protocol.
* `"UDP"` is the UDP protocol.
* **loadBalancer.ingress.ports.error** (string)
Error is to record the problem with the service port The format of the error shall comply with the following rules: - built-in error values shall be specified in this file and those shall use
CamelCase names
* cloud provider specific error values must have names that comply with the
format foo.example.com/CamelCase.## ServiceList
ServiceList holds a list of services.
* **apiVersion**: v1