---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#49-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 69
summary: * **spec.validations.expression** (string), required Expression represents the expression which will be evaluated by CEL. ref: [https://github.com/google/cel-spec](https://github.com/google/cel-spec)...
---

* **spec.validations.expression** (string), required
Expression represents the expression which will be evaluated by CEL. ref: [https://github.com/google/cel-spec](https://github.com/google/cel-spec) CEL expressions have access to the contents of the API request/response, organized into CEL variables as well as some other useful variables: