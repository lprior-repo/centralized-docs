---
doc_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1
chunk_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1#43-summary
chunk_level: summary
chunk_type: prose
heading: ServiceSpec
token_count: 116
summary: * `\"ClientIP\"` is the Client IP based. * `\"None\"` - no session affinity. * **loadBalancerIP** (string) Only applies to Service Type: LoadBalancer. This feature depends on whether the underlying...
---

* `"ClientIP"` is the Client IP based.
* `"None"` - no session affinity.
* **loadBalancerIP** (string)
Only applies to Service Type: LoadBalancer. This feature depends on whether the underlying cloud-provider supports specifying the loadBalancerIP when a load balancer is created. This field will be ignored if the cloud-provider does not support the feature. Deprecated: This field was under-specified and its meaning varies across implementations. Using it is non-portable and it may not support dual-stack. Users are encouraged to use implementation-specific annotations when available.