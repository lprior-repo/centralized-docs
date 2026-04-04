---
doc_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1
chunk_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1#13-summary
chunk_level: summary
chunk_type: prose
heading: ServiceSpec
token_count: 126
summary: * **ports** ([]ServicePort) *Patch strategy: merge on key `port`* *Map: unique values on keys `port, protocol` will be kept during a merge* The list of ports that are exposed by this service. More...
---

* **ports** ([]ServicePort)
*Patch strategy: merge on key `port`*
*Map: unique values on keys `port, protocol` will be kept during a merge*
The list of ports that are exposed by this service. More info: [https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies](https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies)
*ServicePort contains information on service's port.*
* **ports.port** (int32), required
The port that will be exposed by this service.