---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#25-summary
chunk_level: summary
chunk_type: prose
heading: `GroupResources`
token_count: 119
summary: * `\*` matches all resources and their subresources. * `pods/\*` matches all subresources of pods. * `\*/scale` matches all scale subresources. If wildcard is present, the validation rule will ensure...
---

* `\*` matches all resources and their subresources.
* `pods/\*` matches all subresources of pods.
* `\*/scale` matches all scale subresources.
If wildcard is present, the validation rule will ensure resources do not
overlap with each other.
An empty list implies all resources and subresources in this API groups apply.
|
|`resourceNames`
`[]string`|
ResourceNames is a list of resource instance names that the policy matches.
Using this field requires Resources to be specified.
An empty list implies that every instance of the resource is matched.
|