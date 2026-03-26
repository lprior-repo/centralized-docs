---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#42-summary
chunk_level: summary
chunk_type: table
heading: `Connection`
token_count: 123
summary: * [EgressSelection](#apiserver-k8s-io-v1beta1-EgressSelection) Connection provides the configuration for a single egress selection client. |Field|Description| |`proxyProtocol`**[Required]**...
---

* [EgressSelection](#apiserver-k8s-io-v1beta1-EgressSelection)
Connection provides the configuration for a single egress selection client.
|Field|Description|
|`proxyProtocol`**[Required]**
[`ProtocolType`](#apiserver-k8s-io-v1beta1-ProtocolType)|
Protocol is the protocol used to connect from client to the konnectivity server.
|
|`transport`
[`Transport`](#apiserver-k8s-io-v1beta1-Transport)|
Transport defines the transport configurations we use to dial to the konnectivity server.