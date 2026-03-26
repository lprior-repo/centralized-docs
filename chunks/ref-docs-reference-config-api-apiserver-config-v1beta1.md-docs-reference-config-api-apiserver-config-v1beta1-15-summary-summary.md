---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#15-summary
chunk_level: summary
chunk_type: prose
heading: `AuthenticationConfiguration`
token_count: 113
summary: each JWT authenticator must have a unique issuer URL, at most one JWT authenticator will attempt to cryptographically validate the token. The minimum valid JWT payload must contain the following...
---

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