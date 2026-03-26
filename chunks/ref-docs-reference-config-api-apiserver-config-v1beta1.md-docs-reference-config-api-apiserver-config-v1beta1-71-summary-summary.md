---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#71-summary
chunk_level: summary
chunk_type: table
heading: `TCPTransport`
token_count: 127
summary: * [Transport](#apiserver-k8s-io-v1beta1-Transport) TCPTransport provides the information to connect to konnectivity server via TCP |Field|Description| |`url`**[Required]** `string`| URL is the...
---

* [Transport](#apiserver-k8s-io-v1beta1-Transport)
TCPTransport provides the information to connect to konnectivity server via TCP
|Field|Description|
|`url`**[Required]**
`string`|
URL is the location of the konnectivity server to connect to.
As an example it might be "https://127.0.0.1:8131"
|
|`tlsConfig`
[`TLSConfig`](#apiserver-k8s-io-v1beta1-TLSConfig)|
TLSConfig is the config needed to use TLS when connecting to konnectivity server