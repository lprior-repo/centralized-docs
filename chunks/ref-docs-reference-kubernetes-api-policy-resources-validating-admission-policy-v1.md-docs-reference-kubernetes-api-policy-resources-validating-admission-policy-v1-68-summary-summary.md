---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#68-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 123
summary: * **status.conditions.message** (string), required message is a human readable message indicating details about the transition. This may be an empty string. * **status.conditions.reason** (string),...
---

* **status.conditions.message** (string), required
message is a human readable message indicating details about the transition. This may be an empty string.
* **status.conditions.reason** (string), required
reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
* **status.conditions.status** (string), required
status of the condition, one of True, False, Unknown.