---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-api-service-v1.md/docs-reference-kubernetes-api-cluster-resources-api-service-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-api-service-v1.md/docs-reference-kubernetes-api-cluster-resources-api-service-v1#20-summary
chunk_level: summary
chunk_type: prose
heading: APIServiceStatus
token_count: 102
summary: * **conditions** ([]APIServiceCondition) *Patch strategy: merge on key `type`* *Map: unique values on key type will be kept during a merge* Current service state of apiService. *APIServiceCondition...
---

* **conditions** ([]APIServiceCondition)
*Patch strategy: merge on key `type`*
*Map: unique values on key type will be kept during a merge*
Current service state of apiService.
*APIServiceCondition describes the state of an APIService at a particular point*
* **conditions.status** (string), required
Status is the status of the condition. Can be True, False, Unknown.
* **conditions.type** (string), required
Type is the type of the condition.