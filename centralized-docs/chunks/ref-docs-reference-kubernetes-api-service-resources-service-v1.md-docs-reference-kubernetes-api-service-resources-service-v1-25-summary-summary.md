---
doc_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1
chunk_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1#25-summary
chunk_level: summary
chunk_type: prose
heading: ServiceSpec
token_count: 125
summary: * `\"ClusterIP\"` means a service will only be accessible inside the cluster, via the cluster IP. * `\"ExternalName\"` means a service consists of only a reference to an external name that kubedns or...
---

* `"ClusterIP"` means a service will only be accessible inside the cluster, via the cluster IP.
* `"ExternalName"` means a service consists of only a reference to an external name that kubedns or equivalent will return as a CNAME record, with no exposing or proxying of any pods involved.
* `"LoadBalancer"` means a service will be exposed via an external load balancer (if the cloud provider supports it), in addition to 'NodePort' type.
* `"NodePort"` means a service will be exposed on one port of every node, in addition to 'ClusterIP' type.