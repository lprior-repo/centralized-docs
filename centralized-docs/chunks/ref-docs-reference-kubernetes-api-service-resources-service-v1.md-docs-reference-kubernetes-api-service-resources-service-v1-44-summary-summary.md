---
doc_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1
chunk_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1#44-summary
chunk_level: summary
chunk_type: prose
heading: ServiceSpec
token_count: 102
summary: * **loadBalancerSourceRanges** ([]string) *Atomic: will be replaced during a merge* If specified and supported by the platform, this will restrict traffic through the cloud-provider load-balancer...
---

* **loadBalancerSourceRanges** ([]string)
*Atomic: will be replaced during a merge*
If specified and supported by the platform, this will restrict traffic through the cloud-provider load-balancer will be restricted to the specified client IPs. This field will be ignored if the cloud-provider does not support the feature." More info: [https://kubernetes.io/docs/tasks/access-application-cluster/create-external-load-balancer/](https://kubernetes.io/docs/tasks/access-application-cluster/create-external-load-balancer/)