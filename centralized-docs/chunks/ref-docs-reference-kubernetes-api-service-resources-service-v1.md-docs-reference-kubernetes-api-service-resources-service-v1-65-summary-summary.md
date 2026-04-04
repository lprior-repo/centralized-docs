---
doc_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1
chunk_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1#65-summary
chunk_level: summary
chunk_type: prose
heading: ServiceStatus
token_count: 120
summary: * **loadBalancer** (LoadBalancerStatus) LoadBalancer contains the current status of the load-balancer, if one is present. *LoadBalancerStatus represents the status of a load-balancer.* *...
---

* **loadBalancer** (LoadBalancerStatus)
LoadBalancer contains the current status of the load-balancer, if one is present.
*LoadBalancerStatus represents the status of a load-balancer.*
* **loadBalancer.ingress** ([]LoadBalancerIngress)
*Atomic: will be replaced during a merge*
Ingress is a list containing ingress points for the load-balancer. Traffic intended for the service should be sent to these ingress points.
*LoadBalancerIngress represents the status of a load-balancer ingress point: traffic intended for the service should be sent to an ingress point.*