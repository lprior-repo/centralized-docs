---
doc_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1
chunk_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1#15-summary
chunk_level: summary
chunk_type: prose
heading: `AdmissionRequest`
token_count: 113
summary: | |`operation`**[Required]** [`Operation`](#admission-k8s-io-v1-Operation)| operation is the operation being performed. This may be different than the operation requested. e.g. a patch can result in...
---

|
|`operation`**[Required]**
[`Operation`](#admission-k8s-io-v1-Operation)|
operation is the operation being performed. This may be different than the operation
requested. e.g. a patch can result in either a CREATE or UPDATE Operation.
|
|`userInfo`**[Required]**
[`authentication/v1.UserInfo`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.35/#userinfo-v1-authentication-k8s-io)|
userInfo is information about the requesting user
|
|`object`