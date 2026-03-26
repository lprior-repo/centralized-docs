---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#8-detailed
chunk_level: detailed
chunk_type: table
heading: `UserValidationRule`
token_count: 517
summary: ## `Transport` **Appears in:** * [Connection](#apiserver-k8s-io-v1beta1-Connection) Transport defines the transport configurations we use to dial to the konnectivity server |Field|Description| |`tcp`...
---

## `Transport`
**Appears in:**
* [Connection](#apiserver-k8s-io-v1beta1-Connection)
Transport defines the transport configurations we use to dial to the konnectivity server
|Field|Description|
|`tcp`
[`TCPTransport`](#apiserver-k8s-io-v1beta1-TCPTransport)|
TCP is the TCP configuration for communicating with the konnectivity server via TCP
ProxyProtocol of GRPC is not supported with TCP transport at the moment
Requires at least one of TCP or UDS to be set
|
|`uds`
[`UDSTransport`](#apiserver-k8s-io-v1beta1-UDSTransport)|
UDS is the UDS configuration for communicating with the konnectivity server via UDS
Requires at least one of TCP or UDS to be set
|
## `UDSTransport`
**Appears in:**
* [Transport](#apiserver-k8s-io-v1beta1-Transport)
UDSTransport provides the information to connect to konnectivity server via UDS
|Field|Description|
|`udsName`**[Required]**
`string`|
UDSName is the name of the unix domain socket to connect to konnectivity server
This does not use a unix:// prefix. (Eg: /etc/srv/kubernetes/konnectivity-server/konnectivity-server.socket)
|
## `UserValidationRule`
**Appears in:**
* [JWTAuthenticator](#apiserver-k8s-io-v1beta1-JWTAuthenticator)
UserValidationRule provides the configuration for a single user info validation rule.
|Field|Description|
|`expression`**[Required]**
`string`|
expression represents the expression which will be evaluated by CEL.
Must return true for the validation to pass.
CEL expressions have access to the contents of UserInfo, organized into CEL variable:
* 'user' - authentication.k8s.io/v1, Kind=UserInfo object
Refer to https://github.com/kubernetes/api/blob/release-1.28/authentication/v1/types.go#L105-L122 for the definition.
API documentation: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.28/#userinfo-v1-authentication-k8s-io
Documentation on CEL: https://kubernetes.io/docs/reference/using-api/cel/
|
|`message`
`string`|
message customizes the returned error message when rule returns false.
message is a literal string.
|