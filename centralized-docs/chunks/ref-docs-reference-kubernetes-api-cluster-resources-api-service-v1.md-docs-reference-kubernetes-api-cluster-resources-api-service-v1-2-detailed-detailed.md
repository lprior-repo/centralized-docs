---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-api-service-v1.md/docs-reference-kubernetes-api-cluster-resources-api-service-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-api-service-v1.md/docs-reference-kubernetes-api-cluster-resources-api-service-v1#2-detailed
chunk_level: detailed
chunk_type: prose
heading: APIServiceStatus
token_count: 214
summary: ## APIServiceStatus APIServiceStatus contains derived information about an API server * **conditions** ([]APIServiceCondition) *Patch strategy: merge on key `type`* *Map: unique values on key type...
---

## APIServiceStatus
APIServiceStatus contains derived information about an API server
* **conditions** ([]APIServiceCondition)
*Patch strategy: merge on key `type`*
*Map: unique values on key type will be kept during a merge*
Current service state of apiService.
*APIServiceCondition describes the state of an APIService at a particular point*
* **conditions.status** (string), required
Status is the status of the condition. Can be True, False, Unknown.
* **conditions.type** (string), required
Type is the type of the condition.
* **conditions.lastTransitionTime** (Time)
Last time the condition transitioned from one status to another.
*Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON. Wrappers are provided for many of the factory methods that the time package offers.*
* **conditions.message** (string)
Human-readable message indicating details about last transition.
* **conditions.reason** (string)
Unique, one-word, CamelCase reason for the condition's last transition.