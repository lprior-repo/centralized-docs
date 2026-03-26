---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#23-summary
chunk_level: summary
chunk_type: table
heading: `AuthorizerConfiguration`
token_count: 128
summary: * [AuthorizationConfiguration](#apiserver-k8s-io-v1beta1-AuthorizationConfiguration)|Field|Description| |`type`**[Required]** `string`| Type refers to the type of the authorizer \"Webhook\" is...
---

* [AuthorizationConfiguration](#apiserver-k8s-io-v1beta1-AuthorizationConfiguration)|Field|Description|
|`type`**[Required]**
`string`|
Type refers to the type of the authorizer
"Webhook" is supported in the generic API server
Other API servers may support additional authorizer
types like Node, RBAC, ABAC, etc.
|
|`name`**[Required]**
`string`|
Name used to describe the webhook
This is explicitly used in monitoring machinery for metrics
Note: Names must be DNS1123 labels like `myauthorizername`