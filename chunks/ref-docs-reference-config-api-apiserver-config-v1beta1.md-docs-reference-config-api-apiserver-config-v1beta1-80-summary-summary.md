---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#80-summary
chunk_level: summary
chunk_type: table
heading: `UDSTransport`
token_count: 118
summary: ## `UDSTransport` **Appears in:** * [Transport](#apiserver-k8s-io-v1beta1-Transport) UDSTransport provides the information to connect to konnectivity server via UDS |Field|Description|...
---

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