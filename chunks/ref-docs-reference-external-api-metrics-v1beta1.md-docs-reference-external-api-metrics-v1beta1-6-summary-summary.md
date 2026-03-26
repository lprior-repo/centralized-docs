---
doc_id: ref/docs-reference-external-api-metrics-v1beta1.md/docs-reference-external-api-metrics-v1beta1
chunk_id: ref/docs-reference-external-api-metrics-v1beta1.md/docs-reference-external-api-metrics-v1beta1#6-summary
chunk_level: summary
chunk_type: table
heading: Resource Types
token_count: 101
summary: collected from the interval [Timestamp-Window, Timestamp]. | |`window`**[Required]** [`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|No description provided.|...
---

collected from the interval [Timestamp-Window, Timestamp].
|
|`window`**[Required]**
[`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|No description provided.|
|`usage`**[Required]**
[`core/v1.ResourceList`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.28/#resourcelist-v1-core)|
The memory usage is the memory working set.
|