---
doc_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1
chunk_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1#9-detailed
chunk_level: detailed
chunk_type: prose
heading: ServiceStatus
token_count: 1024
summary: * **conditions** ([]Condition) *Patch strategy: merge on key `type`* *Map: unique values on key type will be kept during a merge* Current service state *Condition contains details for one aspect of...
---

* **conditions** ([]Condition)
*Patch strategy: merge on key `type`*
*Map: unique values on key type will be kept during a merge*
Current service state
*Condition contains details for one aspect of the current state of this API Resource.*
* **conditions.lastTransitionTime** (Time), required
lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed. If that is not known, then using the time when the API field changed is acceptable.
*Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON. Wrappers are provided for many of the factory methods that the time package offers.*
* **conditions.message** (string), required
message is a human readable message indicating details about the transition. This may be an empty string.
* **conditions.reason** (string), required
reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
* **conditions.status** (string), required
status of the condition, one of True, False, Unknown.
* **conditions.type** (string), required
type of condition in CamelCase or in foo.example.com/CamelCase.
* **conditions.observedGeneration** (int64)
observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.
* **loadBalancer** (LoadBalancerStatus)
LoadBalancer contains the current status of the load-balancer, if one is present.
*LoadBalancerStatus represents the status of a load-balancer.*
* **loadBalancer.ingress** ([]LoadBalancerIngress)
*Atomic: will be replaced during a merge*
Ingress is a list containing ingress points for the load-balancer. Traffic intended for the service should be sent to these ingress points.
*LoadBalancerIngress represents the status of a load-balancer ingress point: traffic intended for the service should be sent to an ingress point.*
* **loadBalancer.ingress.hostname** (string)
Hostname is set for load-balancer ingress points that are DNS based (typically AWS load-balancers)
* **loadBalancer.ingress.ip** (string)
IP is set for load-balancer ingress points that are IP based (typically GCE or OpenStack load-balancers)
* **loadBalancer.ingress.ipMode** (string)
IPMode specifies how the load-balancer IP behaves, and may only be specified when the ip field is specified. Setting this to "VIP" indicates that traffic is delivered to the node with the destination set to the load-balancer's IP and port. Setting this to "Proxy" indicates that traffic is delivered to the node or pod with the destination set to the node's IP and node port or the pod's IP and port. Service implementations may use this information to adjust traffic routing.
* **loadBalancer.ingress.ports** ([]PortStatus)
*Atomic: will be replaced during a merge*
Ports is a list of records of service ports If used, every port defined in the service should have an entry in it
*PortStatus represents the error condition of a service port*
* **loadBalancer.ingress.ports.port** (int32), required
Port is the port number of the service port of which status is recorded here
* **loadBalancer.ingress.ports.protocol** (string), required
Protocol is the protocol of the service port of which status is recorded here The supported values are: "TCP", "UDP", "SCTP"
Possible enum values:
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
* **kind**: ServiceList
* **metadata** ([ListMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/list-meta/#ListMeta))
Standard list metadata. More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds)