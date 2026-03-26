---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#34-summary
chunk_level: summary
chunk_type: prose
heading: `LeaderElectionConfiguration`
token_count: 94
summary: `string`| resourceLock indicates the resource object type that will be used to lock during leader election cycles. | |`resourceName`**[Required]** `string`| resourceName indicates the name of...
---

`string`|
resourceLock indicates the resource object type that will be used to lock
during leader election cycles.
|
|`resourceName`**[Required]**
`string`|
resourceName indicates the name of resource object that will be used to lock
during leader election cycles.
|
|`resourceNamespace`**[Required]**
`string`|
resourceName indicates the namespace of resource object that will be used to lock
during leader election cycles.
|