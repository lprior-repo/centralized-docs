---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#8-summary
chunk_level: summary
chunk_type: prose
heading: Resource Types
token_count: 123
summary: 3. The remote address for the connection, if it doesn't match the last IP in the list up to here (X-Forwarded-For or X-Real-Ip). Note: All but the last IP can be arbitrarily set by the client.|...
---

3. The remote address for the connection, if it doesn't match the last
IP in the list up to here (X-Forwarded-For or X-Real-Ip).
Note: All but the last IP can be arbitrarily set by the client.|
|`userAgent`
`string`|
UserAgent records the user agent string reported by the client.
Note that the UserAgent is provided by the client, and must not be trusted.
|
|`objectRef`
[`ObjectReference`](#audit-k8s-io-v1-ObjectReference)|
Object reference this request is targeted at.