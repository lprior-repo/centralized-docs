---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#13-summary
chunk_level: summary
chunk_type: table
heading: `AuthenticationConfiguration`
token_count: 122
summary: AuthenticationConfiguration provides versioned configuration for authentication. |Field|Description| |`apiVersion` string|`apiserver.k8s.io/v1beta1`| |`kind` string|`AuthenticationConfiguration`|...
---

AuthenticationConfiguration provides versioned configuration for authentication.
|Field|Description|
|`apiVersion`
string|`apiserver.k8s.io/v1beta1`|
|`kind`
string|`AuthenticationConfiguration`|
|`jwt`**[Required]**
[`[]JWTAuthenticator`](#apiserver-k8s-io-v1beta1-JWTAuthenticator)|
jwt is a list of authenticator to authenticate Kubernetes users using
JWT compliant tokens. The authenticator will attempt to parse a raw ID token,
verify it's been signed by the configured issuer. The public key to verify the