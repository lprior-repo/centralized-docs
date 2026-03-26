---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#64-summary
chunk_level: summary
chunk_type: table
heading: `JWTAuthenticator`
token_count: 128
summary: * [AuthenticationConfiguration](#apiserver-k8s-io-v1beta1-AuthenticationConfiguration) JWTAuthenticator provides the configuration for a single JWT authenticator. |Field|Description|...
---

* [AuthenticationConfiguration](#apiserver-k8s-io-v1beta1-AuthenticationConfiguration)
JWTAuthenticator provides the configuration for a single JWT authenticator.
|Field|Description|
|`issuer`**[Required]**
[`Issuer`](#apiserver-k8s-io-v1beta1-Issuer)|
issuer contains the basic OIDC provider connection options.
|
|`claimValidationRules`
[`[]ClaimValidationRule`](#apiserver-k8s-io-v1beta1-ClaimValidationRule)|
claimValidationRules are rules that are applied to validate token claims to authenticate users.
|
|`claimMappings`