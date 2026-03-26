---
doc_id: ref/docs-reference-external-api-metrics-v1beta1.md/docs-reference-external-api-metrics-v1beta1
chunk_id: ref/docs-reference-external-api-metrics-v1beta1.md/docs-reference-external-api-metrics-v1beta1#5-summary
chunk_level: summary
chunk_type: prose
heading: Resource Types
token_count: 118
summary: | Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata Refer to the Kubernetes API documentation for the fields of the...
---

|
Standard object's metadata.
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
Refer to the Kubernetes API documentation for the fields of the `metadata` field.|
|`timestamp`**[Required]**
[`meta/v1.Time`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.28/#time-v1-meta)|
The following fields define time interval from which metrics were
collected from the interval [Timestamp-Window, Timestamp].
|
|`window`**[Required]**