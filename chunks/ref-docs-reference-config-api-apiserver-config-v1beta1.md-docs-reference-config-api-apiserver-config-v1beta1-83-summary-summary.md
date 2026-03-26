---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#83-summary
chunk_level: summary
chunk_type: prose
heading: `UserValidationRule`
token_count: 119
summary: * 'user' - authentication.k8s.io/v1, Kind=UserInfo object Refer to https://github.com/kubernetes/api/blob/release-1.28/authentication/v1/types.go#L105-L122 for the definition. API documentation:...
---

* 'user' - authentication.k8s.io/v1, Kind=UserInfo object
Refer to https://github.com/kubernetes/api/blob/release-1.28/authentication/v1/types.go#L105-L122 for the definition.
API documentation: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.28/#userinfo-v1-authentication-k8s-io
Documentation on CEL: https://kubernetes.io/docs/reference/using-api/cel/
|
|`message`
`string`|
message customizes the returned error message when rule returns false.
message is a literal string.
|