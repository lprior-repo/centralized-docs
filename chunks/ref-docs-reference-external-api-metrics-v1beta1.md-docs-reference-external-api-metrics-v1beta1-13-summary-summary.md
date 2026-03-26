---
doc_id: ref/docs-reference-external-api-metrics-v1beta1.md/docs-reference-external-api-metrics-v1beta1
chunk_id: ref/docs-reference-external-api-metrics-v1beta1.md/docs-reference-external-api-metrics-v1beta1#13-summary
chunk_level: summary
chunk_type: table
heading: `PodMetrics`
token_count: 93
summary: collected from the interval [Timestamp-Window, Timestamp]. | |`window`**[Required]** [`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|No description provided.|...
---

collected from the interval [Timestamp-Window, Timestamp].
|
|`window`**[Required]**
[`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|No description provided.|
|`containers`**[Required]**
[`[]ContainerMetrics`](#metrics-k8s-io-v1beta1-ContainerMetrics)|
Metrics for all containers are collected within the same time window.
|