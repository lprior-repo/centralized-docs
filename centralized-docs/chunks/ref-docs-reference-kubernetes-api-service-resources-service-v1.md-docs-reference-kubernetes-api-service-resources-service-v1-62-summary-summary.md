---
doc_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1
chunk_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1#62-summary
chunk_level: summary
chunk_type: prose
heading: ServiceStatus
token_count: 119
summary: * **conditions.lastTransitionTime** (Time), required lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed....
---

* **conditions.lastTransitionTime** (Time), required
lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed. If that is not known, then using the time when the API field changed is acceptable.
*Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON. Wrappers are provided for many of the factory methods that the time package offers.*
* **conditions.message** (string), required
message is a human readable message indicating details about the transition. This may be an empty string.