---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#115-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicyBinding
token_count: 126
summary: validationActions is declared as a set of action values. Order does not matter. validationActions may not contain duplicates of the same action. The supported actions values are: \"Deny\" specifies...
---

validationActions is declared as a set of action values. Order does not matter. validationActions may not contain duplicates of the same action.
The supported actions values are:
"Deny" specifies that a validation failure results in a denied request.
"Warn" specifies that a validation failure is reported to the request client in HTTP Warning headers, with a warning code of 299. Warnings can be sent both for allowed or denied admission responses.
"Audit" specifies that a validation failure is included in the published audit event for the request. The audit event will contain a `validation.policy.admission.k8s.io/validation\_failure`