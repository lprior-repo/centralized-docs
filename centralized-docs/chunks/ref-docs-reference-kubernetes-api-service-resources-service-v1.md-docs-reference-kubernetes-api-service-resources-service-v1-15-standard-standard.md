---
doc_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1
chunk_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1#15-standard
chunk_level: standard
chunk_type: prose
heading: ServiceStatus
token_count: 458
summary: * **loadBalancer.ingress.hostname** (string) Hostname is set for load-balancer ingress points that are DNS based (typically AWS load-balancers) * **loadBalancer.ingress.ip** (string) IP is set for...
---

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