---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#1-detailed
chunk_level: detailed
chunk_type: table
heading: Resource Types
token_count: 486
summary: ## Resource Types * [Event](#audit-k8s-io-v1-Event) * [EventList](#audit-k8s-io-v1-EventList) * [Policy](#audit-k8s-io-v1-Policy) * [PolicyList](#audit-k8s-io-v1-PolicyList)## `Event` **Appears in:**...
---

## Resource Types
* [Event](#audit-k8s-io-v1-Event)
* [EventList](#audit-k8s-io-v1-EventList)
* [Policy](#audit-k8s-io-v1-Policy)
* [PolicyList](#audit-k8s-io-v1-PolicyList)## `Event`
**Appears in:**
* [EventList](#audit-k8s-io-v1-EventList)
Event captures all the information that can be included in an API audit log.
|Field|Description|
|`apiVersion`
string|`audit.k8s.io/v1`|
|`kind`
string|`Event`|
|`level`**[Required]**
[`Level`](#audit-k8s-io-v1-Level)|
AuditLevel at which event was generated
|
|`auditID`**[Required]**
[`k8s.io/apimachinery/pkg/types.UID`](https://pkg.go.dev/k8s.io/apimachinery/pkg/types#UID)|
Unique audit ID, generated for each request.
|
|`stage`**[Required]**
[`Stage`](#audit-k8s-io-v1-Stage)|
Stage of the request handling when this event instance was generated.
|
|`requestURI`**[Required]**
`string`|
RequestURI is the request URI as sent by the client to a server.
|
|`verb`**[Required]**
`string`|
Verb is the kubernetes verb associated with the request.
For non-resource requests, this is the lower-cased HTTP method.
|
|`user`**[Required]**
[`authentication/v1.UserInfo`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.35/#userinfo-v1-authentication-k8s-io)|
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