---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#117-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicyBinding
token_count: 125
summary: `\"validation.policy.admission.k8s.io/validation\_failure\": \"[{\\\"message\\\": \\\"Invalid value\\\", {\\\"policy\\\": \\\"policy.example.com\\\", {\\\"binding\\\": \\\"policybinding.example.com\\\",...
---

`"validation.policy.admission.k8s.io/validation\_failure": "[{\\"message\\": \\"Invalid value\\", {\\"policy\\": \\"policy.example.com\\", {\\"binding\\": \\"policybinding.example.com\\", {\\"expressionIndex\\": \\"1\\", {\\"validationActions\\": [\\"Audit\\"]}]"`
Clients should expect to handle additional values by ignoring any values not recognized.
"Deny" and "Warn" may not be used together since this combination needlessly duplicates the validation failure both in the API response body and the HTTP warning headers.
Required.