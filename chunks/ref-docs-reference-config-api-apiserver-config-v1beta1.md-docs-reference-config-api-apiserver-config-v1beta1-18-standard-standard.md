---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#18-standard
chunk_level: standard
chunk_type: table
heading: `UserValidationRule`
token_count: 220
summary: ## `UserValidationRule` **Appears in:** * [JWTAuthenticator](#apiserver-k8s-io-v1beta1-JWTAuthenticator) UserValidationRule provides the configuration for a single user info validation rule....
---

## `UserValidationRule`
**Appears in:**
* [JWTAuthenticator](#apiserver-k8s-io-v1beta1-JWTAuthenticator)
UserValidationRule provides the configuration for a single user info validation rule.
|Field|Description|
|`expression`**[Required]**
`string`|
expression represents the expression which will be evaluated by CEL.
Must return true for the validation to pass.
CEL expressions have access to the contents of UserInfo, organized into CEL variable:
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