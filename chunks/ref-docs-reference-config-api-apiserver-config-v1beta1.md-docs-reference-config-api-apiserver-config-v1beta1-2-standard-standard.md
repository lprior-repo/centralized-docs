---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#2-standard
chunk_level: standard
chunk_type: table
heading: `AuthorizationConfiguration`
token_count: 433
summary: ## `AuthenticationConfiguration` AuthenticationConfiguration provides versioned configuration for authentication. |Field|Description| |`apiVersion` string|`apiserver.k8s.io/v1beta1`| |`kind`...
---

## `AuthenticationConfiguration`
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
signature is discovered from the issuer's public endpoint using OIDC discovery.
For an incoming token, each JWT authenticator will be attempted in
the order in which it is specified in this list. Note however that
other authenticators may run before or after the JWT authenticators.
The specific position of JWT authenticators in relation to other
authenticators is neither defined nor stable across releases. Since
each JWT authenticator must have a unique issuer URL, at most one
JWT authenticator will attempt to cryptographically validate the token.
The minimum valid JWT payload must contain the following claims:
{
"iss": "https://issuer.example.com",
"aud": ["audience"],
"exp": 1234567890,
"": "username"
}
|
|`anonymous`**[Required]**
[`AnonymousAuthConfig`](#apiserver-k8s-io-v1beta1-AnonymousAuthConfig)|
If present --anonymous-auth must not be set
|
## `AuthorizationConfiguration`
|Field|Description|
|`apiVersion`
string|`apiserver.k8s.io/v1beta1`|
|`kind`
string|`AuthorizationConfiguration`|
|`authorizers`**[Required]**
[`[]AuthorizerConfiguration`](#apiserver-k8s-io-v1beta1-AuthorizerConfiguration)|
Authorizers is an ordered list of authorizers to
authorize requests against.
This is similar to the --authorization-modes kube-apiserver flag
Must be at least one.
|