---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#94-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicyBinding
token_count: 48
summary: * `\"Equivalent\"` means requests should be sent to the webhook if they modify a resource listed in rules via another API group or version. * `\"Exact\"` means requests should only be sent to the webhook...
---

* `"Equivalent"` means requests should be sent to the webhook if they modify a resource listed in rules via another API group or version.
* `"Exact"` means requests should only be sent to the webhook if they exactly match a given rule.