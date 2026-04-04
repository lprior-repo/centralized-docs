---
doc_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1
chunk_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1#24-summary
chunk_level: summary
chunk_type: prose
heading: ServiceSpec
token_count: 91
summary: \"ExternalName\" aliases this service to the specified externalName. Several other fields do not apply to ExternalName services. More info:...
---

"ExternalName" aliases this service to the specified externalName. Several other fields do not apply to ExternalName services. More info: [https://kubernetes.io/docs/concepts/services-networking/service/#publishing-services-service-types](https://kubernetes.io/docs/concepts/services-networking/service/#publishing-services-service-types)
Possible enum values:
* `"ClusterIP"` means a service will only be accessible inside the cluster, via the cluster IP.