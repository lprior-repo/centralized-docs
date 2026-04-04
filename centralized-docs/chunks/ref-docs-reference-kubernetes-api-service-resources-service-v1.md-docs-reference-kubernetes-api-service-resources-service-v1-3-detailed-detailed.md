---
doc_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1
chunk_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1#3-detailed
chunk_level: detailed
chunk_type: prose
heading: ServiceSpec
token_count: 1011
summary: * **selector** (map[string]string) Route service traffic to pods with label keys and values matching this selector. If empty or not present, the service is assumed to have an external process...
---

* **selector** (map[string]string)
Route service traffic to pods with label keys and values matching this selector. If empty or not present, the service is assumed to have an external process managing its endpoints, which Kubernetes will not modify. Only applies to types ClusterIP, NodePort, and LoadBalancer. Ignored if type is ExternalName. More info: [https://kubernetes.io/docs/concepts/services-networking/service/](https://kubernetes.io/docs/concepts/services-networking/service/)
* **ports** ([]ServicePort)
*Patch strategy: merge on key `port`*
*Map: unique values on keys `port, protocol` will be kept during a merge*
The list of ports that are exposed by this service. More info: [https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies](https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies)
*ServicePort contains information on service's port.*
* **ports.port** (int32), required
The port that will be exposed by this service.
* **ports.targetPort** (IntOrString)
Number or name of the port to access on the pods targeted by the service. Number must be in the range 1 to 65535. Name must be an IANA\_SVC\_NAME. If this is a string, it will be looked up as a named port in the target Pod's container ports. If this is not specified, the value of the 'port' field is used (an identity map). This field is ignored for services with clusterIP=None, and should be omitted or set equal to the 'port' field. More info: [https://kubernetes.io/docs/concepts/services-networking/service/#defining-a-service](https://kubernetes.io/docs/concepts/services-networking/service/#defining-a-service)
*IntOrString is a type that can hold an int32 or a string. When used in JSON or YAML marshalling and unmarshalling, it produces or consumes the inner type. This allows you to have, for example, a JSON field that can accept a name or number.*
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
* 'kubernetes.io/wss' - WebSocket over TLS as described in [https://www.rfc-editor.org/rfc/rfc6455](https://www.rfc-editor.org/rfc/rfc6455)
* Other protocols should use implementation-defined prefixed names such as mycompany.com/my-custom-protocol.