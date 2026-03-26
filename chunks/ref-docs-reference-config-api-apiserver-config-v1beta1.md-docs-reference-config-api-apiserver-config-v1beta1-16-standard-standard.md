---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#16-standard
chunk_level: standard
chunk_type: table
heading: `TLSConfig`
token_count: 389
summary: ## `TCPTransport` **Appears in:** * [Transport](#apiserver-k8s-io-v1beta1-Transport) TCPTransport provides the information to connect to konnectivity server via TCP |Field|Description|...
---

## `TCPTransport`
**Appears in:**
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
|
## `TLSConfig`
**Appears in:**
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
clientKey is the file location of the client key to be used in mtls handshakes with the konnectivity server.
Must be absent/empty if TCPTransport.URL is prefixed with http://
Must be configured if TCPTransport.URL is prefixed with https://
|
|`clientCert`
`string`|
clientCert is the file location of the client certificate to be used in mtls handshakes with the konnectivity server.
Must be absent/empty if TCPTransport.URL is prefixed with http://
Must be configured if TCPTransport.URL is prefixed with https://
|