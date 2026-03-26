---
doc_id: tutorial/docs-reference-external-api-external-metrics-v1beta1.md/docs-reference-external-api-external-metrics-v1beta1
chunk_id: tutorial/docs-reference-external-api-external-metrics-v1beta1.md/docs-reference-external-api-external-metrics-v1beta1#6-summary
chunk_level: summary
chunk_type: prose
heading: Resource Types
token_count: 99
summary: | |`window`**[Required]** `int64`| indicates the window ([Timestamp-Window, Timestamp]) from which these metrics were calculated, when returning rate metrics calculated from cumulative metrics (or...
---

|
|`window`**[Required]**
`int64`|
indicates the window ([Timestamp-Window, Timestamp]) from
which these metrics were calculated, when returning rate
metrics calculated from cumulative metrics (or zero for
non-calculated instantaneous metrics).
|
|`value`**[Required]**
[`k8s.io/apimachinery/pkg/api/resource.Quantity`](https://pkg.go.dev/k8s.io/apimachinery/pkg/api/resource#Quantity)|
the value of the metric
|