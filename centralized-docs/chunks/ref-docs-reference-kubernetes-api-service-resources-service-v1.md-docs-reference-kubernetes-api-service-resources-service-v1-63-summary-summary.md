---
doc_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1
chunk_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1#63-summary
chunk_level: summary
chunk_type: prose
heading: ServiceStatus
token_count: 120
summary: * **conditions.message** (string), required message is a human readable message indicating details about the transition. This may be an empty string. * **conditions.reason** (string), required reason...
---

* **conditions.message** (string), required
message is a human readable message indicating details about the transition. This may be an empty string.
* **conditions.reason** (string), required
reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
* **conditions.status** (string), required
status of the condition, one of True, False, Unknown.