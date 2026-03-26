---
doc_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1
chunk_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1#12-summary
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
|`options`**[Required]**
[`[]CommandOptionDefault`](#kubectl-config-k8s-io-v1beta1-CommandOptionDefault)|
options is allocated to store the option definitions of alias.