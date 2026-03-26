---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#53-summary
chunk_level: summary
chunk_type: prose
heading: `Issuer`
token_count: 125
summary: --oidc-issuer-url flag. Discovery information is fetched from \"{url}/.well-known/openid-configuration\" unless overridden by discoveryURL. Required to be unique across all JWT authenticators. Note...
---

--oidc-issuer-url flag.
Discovery information is fetched from "{url}/.well-known/openid-configuration" unless overridden by discoveryURL.
Required to be unique across all JWT authenticators.
Note that egress selection configuration is not used for this network connection.
|
|`discoveryURL`
`string`|
discoveryURL, if specified, overrides the URL used to fetch discovery
information instead of using "{url}/.well-known/openid-configuration".
The exact value specified is used, so "/.well-known/openid-configuration"
must be included in discoveryURL if needed.
The "issuer"