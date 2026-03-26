---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#38-summary
chunk_level: summary
chunk_type: table
heading: `ClaimValidationRule`
token_count: 119
summary: * [JWTAuthenticator](#apiserver-k8s-io-v1beta1-JWTAuthenticator) ClaimValidationRule provides the configuration for a single claim validation rule. |Field|Description| |`claim` `string`| claim is the...
---

* [JWTAuthenticator](#apiserver-k8s-io-v1beta1-JWTAuthenticator)
ClaimValidationRule provides the configuration for a single claim validation rule.
|Field|Description|
|`claim`
`string`|
claim is the name of a required claim.
Same as --oidc-required-claim flag.
Only string claim keys are supported.
Mutually exclusive with expression and message.
|
|`requiredValue`
`string`|
requiredValue is the value of a required claim.
Same as --oidc-required-claim flag.
Only string claim values are supported.