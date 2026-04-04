---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-api-service-v1.md/docs-reference-kubernetes-api-cluster-resources-api-service-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-api-service-v1.md/docs-reference-kubernetes-api-cluster-resources-api-service-v1#21-summary
chunk_level: summary
chunk_type: prose
heading: APIServiceStatus
token_count: 114
summary: * **conditions.type** (string), required Type is the type of the condition. * **conditions.lastTransitionTime** (Time) Last time the condition transitioned from one status to another. *Time is a...
---

* **conditions.type** (string), required
Type is the type of the condition.
* **conditions.lastTransitionTime** (Time)
Last time the condition transitioned from one status to another.
*Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON. Wrappers are provided for many of the factory methods that the time package offers.*
* **conditions.message** (string)
Human-readable message indicating details about last transition.
* **conditions.reason** (string)
Unique, one-word, CamelCase reason for the condition's last transition.