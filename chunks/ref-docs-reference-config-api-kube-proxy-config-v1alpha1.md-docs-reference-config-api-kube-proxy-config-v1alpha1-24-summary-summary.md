---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#24-summary
chunk_level: summary
chunk_type: prose
heading: `ClientConnectionConfiguration`
token_count: 113
summary: default value of 'application/json'. This field will control all connections to the server used by a particular client. | |`contentType`**[Required]** `string`| contentType is the content type used...
---

default value of 'application/json'. This field will control all connections to the server used by a particular
client.
|
|`contentType`**[Required]**
`string`|
contentType is the content type used when sending data to the server from this client.
|
|`qps`**[Required]**
`float32`|
qps controls the number of queries per second allowed for this connection.
|
|`burst`**[Required]**
`int32`|
burst allows extra queries to accumulate when a client is exceeding its rate.
|