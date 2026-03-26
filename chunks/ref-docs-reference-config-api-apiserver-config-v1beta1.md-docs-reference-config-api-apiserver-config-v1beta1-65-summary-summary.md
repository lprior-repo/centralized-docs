---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#65-summary
chunk_level: summary
chunk_type: prose
heading: `JWTAuthenticator`
token_count: 121
summary: | claimValidationRules are rules that are applied to validate token claims to authenticate users. | |`claimMappings`**[Required]** [`ClaimMappings`](#apiserver-k8s-io-v1beta1-ClaimMappings)|...
---

|
claimValidationRules are rules that are applied to validate token claims to authenticate users.
|
|`claimMappings`**[Required]**
[`ClaimMappings`](#apiserver-k8s-io-v1beta1-ClaimMappings)|
claimMappings points claims of a token to be treated as user attributes.
|
|`userValidationRules`
[`[]UserValidationRule`](#apiserver-k8s-io-v1beta1-UserValidationRule)|
userValidationRules are rules that are applied to final user before completing authentication.
These allow invariants to be applied to incoming identities such as preventing the