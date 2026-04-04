---
doc_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1
chunk_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1#10-summary
chunk_level: summary
chunk_type: prose
heading: `AdmissionRequest`
token_count: 113
summary: with `kind: {group:\"apps\", version:\"v1\", kind:\"Deployment\"}` (matching the rule the webhook registered for), and `requestKind: {group:\"apps\", version:\"v1beta1\", kind:\"Deployment\"}` (indicating the...
---

with `kind: {group:"apps", version:"v1", kind:"Deployment"}` (matching the rule the webhook registered for),
and `requestKind: {group:"apps", version:"v1beta1", kind:"Deployment"}` (indicating the kind of the original API request).
See documentation for the "matchPolicy" field in the webhook configuration type for more details.
|
|`requestResource`
[`meta/v1.GroupVersionResource`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#GroupVersionResource)|