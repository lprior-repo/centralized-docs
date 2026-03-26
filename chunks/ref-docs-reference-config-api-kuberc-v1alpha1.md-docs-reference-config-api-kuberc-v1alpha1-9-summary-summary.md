---
doc_id: ref/docs-reference-config-api-kuberc-v1alpha1.md/docs-reference-config-api-kuberc-v1alpha1
chunk_id: ref/docs-reference-config-api-kuberc-v1alpha1.md/docs-reference-config-api-kuberc-v1alpha1#9-summary
chunk_level: summary
chunk_type: prose
heading: `AliasOverride`
token_count: 127
summary: \"set env\" or \"create\" | |`prependArgs`**[Required]** `[]string`| prependArgs stores the arguments such as resource names, etc. These arguments are inserted after the alias name. |...
---

"set env" or "create"
|
|`prependArgs`**[Required]**
`[]string`|
prependArgs stores the arguments such as resource names, etc.
These arguments are inserted after the alias name.
|
|`appendArgs`**[Required]**
`[]string`|
appendArgs stores the arguments such as resource names, etc.
These arguments are appended to the USER\_ARGS.
|
|`flags`**[Required]**
[`[]CommandOptionDefault`](#kubectl-config-k8s-io-v1alpha1-CommandOptionDefault)|
flags is allocated to store the flag definitions of alias.