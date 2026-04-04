---
doc_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1
chunk_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1#4-standard
chunk_level: standard
chunk_type: prose
heading: ServiceSpec
token_count: 506
summary: * **ports.protocol** (string) The IP protocol for this port. Supports \"TCP\", \"UDP\", and \"SCTP\". Default is TCP. Possible enum values: * `\"SCTP\"` is the SCTP protocol. * `\"TCP\"` is the TCP protocol. *...
---

* **ports.protocol** (string)
The IP protocol for this port. Supports "TCP", "UDP", and "SCTP". Default is TCP.
Possible enum values:
* `"SCTP"` is the SCTP protocol.
* `"TCP"` is the TCP protocol.
* `"UDP"` is the UDP protocol.
* **ports.name** (string)
The name of this port within the service. This must be a DNS\_LABEL. All ports within a ServiceSpec must have unique names. When considering the endpoints for a Service, this must match the 'name' field in the EndpointPort. Optional if only one ServicePort is defined on this service.
* **ports.nodePort** (int32)
The port on each node on which this service is exposed when type is NodePort or LoadBalancer. Usually assigned by the system. If a value is specified, in-range, and not in use it will be used, otherwise the operation will fail. If not specified, a port will be allocated if this Service requires one. If this field is specified when creating a Service which does not need it, creation will fail. This field will be wiped when updating a Service to no longer need it (e.g. changing type from NodePort to ClusterIP). More info: [https://kubernetes.io/docs/concepts/services-networking/service/#type-nodeport](https://kubernetes.io/docs/concepts/services-networking/service/#type-nodeport)
* **ports.appProtocol** (string)
The application protocol for this port. This is used as a hint for implementations to offer richer behavior for protocols that they understand. This field follows standard Kubernetes label syntax. Valid values are either:
* Un-prefixed protocol names - reserved for IANA standard service names (as per RFC-6335 and [https://www.iana.org/assignments/service-names)](https://www.iana.org/assignments/service-names)).
* Kubernetes-defined prefixed names:
* 'kubernetes.io/h2c' - HTTP/2 prior knowledge over cleartext as described in [https://www.rfc-editor.org/rfc/rfc9113.html#name-starting-http-2-with-prior-](https://www.rfc-editor.org/rfc/rfc9113.html#name-starting-http-2-with-prior-)
* 'kubernetes.io/ws' - WebSocket over cleartext as described in [https://www.rfc-editor.org/rfc/rfc6455](https://www.rfc-editor.org/rfc/rfc6455)