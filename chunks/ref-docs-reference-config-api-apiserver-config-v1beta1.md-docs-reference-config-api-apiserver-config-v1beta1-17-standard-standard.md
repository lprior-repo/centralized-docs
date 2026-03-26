---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#17-standard
chunk_level: standard
chunk_type: table
heading: `UDSTransport`
token_count: 297
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