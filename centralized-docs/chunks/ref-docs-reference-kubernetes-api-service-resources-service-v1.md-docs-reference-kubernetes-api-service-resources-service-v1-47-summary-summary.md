---
doc_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1
chunk_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1#47-summary
chunk_level: summary
chunk_type: prose
heading: ServiceSpec
token_count: 117
summary: 'LoadBalancer'. Once set, it can not be changed. This field will be wiped when a service is updated to a non 'LoadBalancer' type. * **externalName** (string) externalName is the external reference...
---

'LoadBalancer'. Once set, it can not be changed. This field will be wiped when a service is updated to a non 'LoadBalancer' type.
* **externalName** (string)
externalName is the external reference that discovery mechanisms will return as an alias for this service (e.g. a DNS CNAME record). No proxying will be involved. Must be a lowercase RFC-1123 hostname ([https://tools.ietf.org/html/rfc1123](https://tools.ietf.org/html/rfc1123)) and requires `type` to be "ExternalName".