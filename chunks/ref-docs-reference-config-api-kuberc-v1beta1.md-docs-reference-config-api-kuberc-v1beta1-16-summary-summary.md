---
doc_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1
chunk_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1#16-summary
chunk_level: summary
chunk_type: table
heading: `AllowlistEntry`
token_count: 107
summary: the logical AND of all checks corresponding to the specified fields within the entry. |Field|Description| |`name`**[Required]** `string`| Name matching is performed by first resolving the absolute...
---

the logical AND of all checks corresponding to the specified fields within
the entry.
|Field|Description|
|`name`**[Required]**
`string`|
Name matching is performed by first resolving the absolute path of both
the plugin and the name in the allowlist entry using `exec.LookPath`. It
will be called on both, and the resulting strings must be equal. If
either call to `exec.LookPath` results in an error, the `Name` check
will be considered a failure.
|