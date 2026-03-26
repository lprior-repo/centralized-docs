---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#15-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 46
summary: Allowed values are Ignore or Fail. Defaults to Fail. Possible enum values: * `\"Fail\"` means that an error calling the webhook causes the admission to fail. * `\"Ignore\"` means that an error calling...
---

Allowed values are Ignore or Fail. Defaults to Fail.
Possible enum values:
* `"Fail"` means that an error calling the webhook causes the admission to fail.
* `"Ignore"` means that an error calling the webhook is ignored.