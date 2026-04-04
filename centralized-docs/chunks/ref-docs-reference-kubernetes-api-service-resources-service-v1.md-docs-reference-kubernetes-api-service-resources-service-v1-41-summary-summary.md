---
doc_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1
chunk_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1#41-summary
chunk_level: summary
chunk_type: prose
heading: ServiceSpec
token_count: 83
summary: * **externalIPs** ([]string) *Atomic: will be replaced during a merge* externalIPs is a list of IP addresses for which nodes in the cluster will also accept traffic for this service. These IPs are...
---

* **externalIPs** ([]string)
*Atomic: will be replaced during a merge*
externalIPs is a list of IP addresses for which nodes in the cluster will also accept traffic for this service. These IPs are not managed by Kubernetes. The user is responsible for ensuring that traffic arrives at a node with this IP. A common example is external load-balancers that are not part of the Kubernetes system.