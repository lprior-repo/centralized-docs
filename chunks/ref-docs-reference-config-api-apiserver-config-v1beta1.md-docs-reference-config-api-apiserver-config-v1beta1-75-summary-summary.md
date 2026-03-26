---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#75-summary
chunk_level: summary
chunk_type: prose
heading: `TLSConfig`
token_count: 118
summary: If absent while TCPTransport.URL is prefixed with https://, default to system trust roots. | |`clientKey` `string`| clientKey is the file location of the client key to be used in mtls handshakes with...
---

If absent while TCPTransport.URL is prefixed with https://, default to system trust roots.
|
|`clientKey`
`string`|
clientKey is the file location of the client key to be used in mtls handshakes with the konnectivity server.
Must be absent/empty if TCPTransport.URL is prefixed with http://
Must be configured if TCPTransport.URL is prefixed with https://
|
|`clientCert`
`string`|
clientCert is the file location of the client certificate to be used in mtls handshakes with the konnectivity server.