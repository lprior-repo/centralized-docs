---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#17-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 80
summary: If a parameter object is provided, it can be accessed via the `params` handle in the same manner as validation expressions. The exact matching logic is (in order): 1. If ANY matchCondition evaluates...
---

If a parameter object is provided, it can be accessed via the `params` handle in the same manner as validation expressions.
The exact matching logic is (in order):
1. If ANY matchCondition evaluates to FALSE, the policy is skipped.
2. If ALL matchConditions evaluate to TRUE, the policy is evaluated.
3. If any matchCondition evaluates to an error (but none are FALSE):