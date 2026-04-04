---
doc_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1
chunk_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1#3-standard
chunk_level: standard
chunk_type: prose
heading: ServiceSpec
token_count: 512
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