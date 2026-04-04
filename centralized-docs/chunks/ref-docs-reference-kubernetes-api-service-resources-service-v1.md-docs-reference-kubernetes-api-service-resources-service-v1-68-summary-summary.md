---
doc_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1
chunk_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1#68-summary
chunk_level: summary
chunk_type: prose
heading: ServiceStatus
token_count: 93
summary: * **loadBalancer.ingress.ports** ([]PortStatus) *Atomic: will be replaced during a merge* Ports is a list of records of service ports If used, every port defined in the service should have an entry...
---

* **loadBalancer.ingress.ports** ([]PortStatus)
*Atomic: will be replaced during a merge*
Ports is a list of records of service ports If used, every port defined in the service should have an entry in it
*PortStatus represents the error condition of a service port*
* **loadBalancer.ingress.ports.port** (int32), required
Port is the port number of the service port of which status is recorded here