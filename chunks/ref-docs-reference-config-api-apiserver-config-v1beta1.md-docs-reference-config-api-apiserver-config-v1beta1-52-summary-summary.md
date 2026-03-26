---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#52-summary
chunk_level: summary
chunk_type: table
heading: `Issuer`
token_count: 124
summary: * [JWTAuthenticator](#apiserver-k8s-io-v1beta1-JWTAuthenticator) Issuer provides the configuration for an external provider's specific settings. |Field|Description| |`url`**[Required]** `string`| url...
---

* [JWTAuthenticator](#apiserver-k8s-io-v1beta1-JWTAuthenticator)
Issuer provides the configuration for an external provider's specific settings.
|Field|Description|
|`url`**[Required]**
`string`|
url points to the issuer URL in a format https://url or https://url/path.
This must match the "iss" claim in the presented JWT, and the issuer returned from discovery.
Same value as the --oidc-issuer-url flag.
Discovery information is fetched from "{url}/.well-known/openid-configuration" unless overridden by discoveryURL.