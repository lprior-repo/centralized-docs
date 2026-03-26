---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#57-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 117
summary: * 'set': `X + Y` performs a union where the array positions of all elements in `X` are preserved and non-intersecting elements in `Y` are appended, retaining their partial order. * 'map': `X + Y`...
---

* 'set': `X + Y` performs a union where the array positions of all elements in `X` are preserved and
non-intersecting elements in `Y` are appended, retaining their partial order.
* 'map': `X + Y` performs a merge where the array positions of all keys in `X` are preserved but the values
are overwritten by values in `Y` when the key sets of `X` and `Y` intersect. Elements in `Y` with
non-intersecting keys are appended, retaining their partial order.
Required.