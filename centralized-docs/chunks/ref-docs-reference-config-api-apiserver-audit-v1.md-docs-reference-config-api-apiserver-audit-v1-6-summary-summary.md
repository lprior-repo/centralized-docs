---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#6-summary
chunk_level: summary
chunk_type: prose
heading: Resource Types
token_count: 119
summary: | Authenticated user information. | |`impersonatedUser` [`authentication/v1.UserInfo`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.35/#userinfo-v1-authentication-k8s-io)|...
---

|
Authenticated user information.
|
|`impersonatedUser`
[`authentication/v1.UserInfo`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.35/#userinfo-v1-authentication-k8s-io)|
Impersonated user information.
|
|`authenticationMetadata`
[`AuthenticationMetadata`](#audit-k8s-io-v1-AuthenticationMetadata)|
AuthenticationMetadata contains details about how the request was authenticated.
|
|`sourceIPs`
`[]string`|
Source IPs, from where the request originated and intermediate proxies.
The source IPs are listed from (in order):