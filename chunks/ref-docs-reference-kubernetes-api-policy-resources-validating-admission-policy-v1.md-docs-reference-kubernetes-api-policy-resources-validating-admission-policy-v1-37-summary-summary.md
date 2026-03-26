---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#37-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 94
summary: namespaceSelector\": { \"matchExpressions\": [ { \"key\": \"environment\", \"operator\": \"In\", \"values\": [ \"prod\", \"staging\" ] } ] } See...
---

namespaceSelector": {
"matchExpressions": [
{
"key": "environment",
"operator": "In",
"values": [
"prod",
"staging"
]
}
]
}
See [https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/](https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/) for more examples of label selectors.
Default to the empty LabelSelector, which matches everything.