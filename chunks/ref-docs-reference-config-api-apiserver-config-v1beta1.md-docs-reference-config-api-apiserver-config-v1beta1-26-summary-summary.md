---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#26-summary
chunk_level: summary
chunk_type: table
heading: `ClaimMappings`
token_count: 128
summary: * [JWTAuthenticator](#apiserver-k8s-io-v1beta1-JWTAuthenticator) ClaimMappings provides the configuration for claim mapping |Field|Description| |`username`**[Required]**...
---

* [JWTAuthenticator](#apiserver-k8s-io-v1beta1-JWTAuthenticator)
ClaimMappings provides the configuration for claim mapping
|Field|Description|
|`username`**[Required]**
[`PrefixedClaimOrExpression`](#apiserver-k8s-io-v1beta1-PrefixedClaimOrExpression)|
username represents an option for the username attribute.
The claim's value must be a singular string.
Same as the --oidc-username-claim and --oidc-username-prefix flags.
If username.expression is set, the expression must produce a string value.
If username.expression uses '