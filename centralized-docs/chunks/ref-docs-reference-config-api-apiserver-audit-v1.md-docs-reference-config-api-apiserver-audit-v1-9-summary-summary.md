---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#9-summary
chunk_level: summary
chunk_type: prose
heading: Resource Types
token_count: 126
summary: |`objectRef` [`ObjectReference`](#audit-k8s-io-v1-ObjectReference)| Object reference this request is targeted at. Does not apply for List-type requests, or non-resource requests. | |`responseStatus`...
---

|`objectRef`
[`ObjectReference`](#audit-k8s-io-v1-ObjectReference)|
Object reference this request is targeted at.
Does not apply for List-type requests, or non-resource requests.
|
|`responseStatus`
[`meta/v1.Status`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.35/#status-v1-meta)|
The response status, populated even when the ResponseObject is not a Status type.
For successful responses, this will only include the Code and StatusSuccess.
For non-status type error responses, this will be auto-populated with the error Message.
|
|