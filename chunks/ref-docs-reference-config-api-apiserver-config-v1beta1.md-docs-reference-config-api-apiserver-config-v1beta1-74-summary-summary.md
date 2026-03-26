---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#74-summary
chunk_level: summary
chunk_type: table
heading: `TLSConfig`
token_count: 119
summary: * [TCPTransport](#apiserver-k8s-io-v1beta1-TCPTransport) TLSConfig provides the authentication information to connect to konnectivity server Only used with TCPTransport |Field|Description|...
---

* [TCPTransport](#apiserver-k8s-io-v1beta1-TCPTransport)
TLSConfig provides the authentication information to connect to konnectivity server
Only used with TCPTransport
|Field|Description|
|`caBundle`
`string`|
caBundle is the file location of the CA to be used to determine trust with the konnectivity server.
Must be absent/empty if TCPTransport.URL is prefixed with http://
If absent while TCPTransport.URL is prefixed with https://, default to system trust roots.
|
|`clientKey`
`string`|