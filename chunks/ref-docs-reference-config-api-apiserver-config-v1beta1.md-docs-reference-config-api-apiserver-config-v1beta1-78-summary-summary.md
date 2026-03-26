---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#78-summary
chunk_level: summary
chunk_type: table
heading: `Transport`
token_count: 112
summary: * [Connection](#apiserver-k8s-io-v1beta1-Connection) Transport defines the transport configurations we use to dial to the konnectivity server |Field|Description| |`tcp`...
---

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