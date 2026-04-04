---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#2-detailed
chunk_level: detailed
chunk_type: table
heading: `EventList`
token_count: 749
summary: 1. X-Forwarded-For request header IPs 2. X-Real-Ip header, if not present in the X-Forwarded-For list 3. The remote address for the connection, if it doesn't match the last IP in the list up to here...
---

1. X-Forwarded-For request header IPs
2. X-Real-Ip header, if not present in the X-Forwarded-For list
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
Does not apply for List-type requests, or non-resource requests.
|
|`responseStatus`
[`meta/v1.Status`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.35/#status-v1-meta)|
The response status, populated even when the ResponseObject is not a Status type.
For successful responses, this will only include the Code and StatusSuccess.
For non-status type error responses, this will be auto-populated with the error Message.
|
|`requestObject`
[`k8s.io/apimachinery/pkg/runtime.Unknown`](https://pkg.go.dev/k8s.io/apimachinery/pkg/runtime#Unknown)|
API object from the request, in JSON format. The RequestObject is recorded as-is in the request
(possibly re-encoded as JSON), prior to version conversion, defaulting, admission or
merging. It is an external versioned object type, and may not be a valid object on its own.
Omitted for non-resource requests. Only logged at Request Level and higher.
|
|`responseObject`
[`k8s.io/apimachinery/pkg/runtime.Unknown`](https://pkg.go.dev/k8s.io/apimachinery/pkg/runtime#Unknown)|
API object returned in the response, in JSON. The ResponseObject is recorded after conversion
to the external type, and serialized as JSON. Omitted for non-resource requests. Only logged
at Response Level.
|
|`requestReceivedTimestamp`
[`meta/v1.MicroTime`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.35/#microtime-v1-meta)|
Time the request reached the apiserver.
|
|`stageTimestamp`
[`meta/v1.MicroTime`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.35/#microtime-v1-meta)|
Time the request reached current audit stage.
|
|`annotations`
`map[string]string`|
Annotations is an unstructured key value map stored with an audit event that may be set by
plugins invoked in the request serving chain, including authentication, authorization and
admission plugins. Note that these annotations are for the audit event, and do not correspond
to the metadata.annotations of the submitted object. Keys should uniquely identify the informing
component to avoid name collisions (e.g. podsecuritypolicy.admission.k8s.io/policy). Values
should be short. Annotations are included in the Metadata level.
|
## `EventList`
EventList is a list of audit Events.
|Field|Description|
|`apiVersion`
string|`audit.k8s.io/v1`|
|`kind`
string|`EventList`|
|`metadata`
[`meta/v1.ListMeta`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.35/#listmeta-v1-meta)|No description provided.|
|`items`**[Required]**
[`[]Event`](#audit-k8s-io-v1-Event)|No description provided.|