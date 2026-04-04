---
doc_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1
chunk_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1#6-summary
chunk_level: summary
chunk_type: prose
heading: `AdmissionRequest`
token_count: 128
summary: otherwise identical (parallel requests, requests when earlier requests did not modify etc) The UID is meant to track the round trip (request/response) between the KAS and the WebHook, not the user...
---

otherwise identical (parallel requests, requests when earlier requests did not modify etc)
The UID is meant to track the round trip (request/response) between the KAS and the WebHook, not the user request.
It is suitable for correlating log entries between the webhook and apiserver, for either auditing or debugging.
|
|`kind`**[Required]**
[`meta/v1.GroupVersionKind`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#GroupVersionKind)|
kind is the fully-qualified type of object being submitted (for example, v1.Pod or autoscaling.v1.Scale)
|