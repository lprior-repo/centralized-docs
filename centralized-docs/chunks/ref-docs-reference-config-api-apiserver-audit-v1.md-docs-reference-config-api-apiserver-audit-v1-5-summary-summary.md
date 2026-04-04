---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#5-summary
chunk_level: summary
chunk_type: prose
heading: Resource Types
token_count: 117
summary: **[Required]** `string`| RequestURI is the request URI as sent by the client to a server. | |`verb`**[Required]** `string`| Verb is the kubernetes verb associated with the request. For non-resource...
---

**[Required]**
`string`|
RequestURI is the request URI as sent by the client to a server.
|
|`verb`**[Required]**
`string`|
Verb is the kubernetes verb associated with the request.
For non-resource requests, this is the lower-cased HTTP method.
|
|`user`**[Required]**
[`authentication/v1.UserInfo`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.35/#userinfo-v1-authentication-k8s-io)|
Authenticated user information.
|
|`impersonatedUser`