---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#82-summary
chunk_level: summary
chunk_type: table
heading: `UserValidationRule`
token_count: 89
summary: * [JWTAuthenticator](#apiserver-k8s-io-v1beta1-JWTAuthenticator) UserValidationRule provides the configuration for a single user info validation rule. |Field|Description| |`expression`**[Required]**...
---

* [JWTAuthenticator](#apiserver-k8s-io-v1beta1-JWTAuthenticator)
UserValidationRule provides the configuration for a single user info validation rule.
|Field|Description|
|`expression`**[Required]**
`string`|
expression represents the expression which will be evaluated by CEL.
Must return true for the validation to pass.
CEL expressions have access to the contents of UserInfo, organized into CEL variable: