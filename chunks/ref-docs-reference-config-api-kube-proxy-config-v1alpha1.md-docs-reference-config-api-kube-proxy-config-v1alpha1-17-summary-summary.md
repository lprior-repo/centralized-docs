---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#17-summary
chunk_level: summary
chunk_type: prose
heading: `OutputRoutingOptions`
token_count: 101
summary: both to stdout, without buffering. Only available when the LoggingAlphaOptions feature gate is enabled. | |`infoBufferSize`**[Required]**...
---

both to stdout, without buffering. Only available when
the LoggingAlphaOptions feature gate is enabled.
|
|`infoBufferSize`**[Required]**
[`k8s.io/apimachinery/pkg/api/resource.QuantityValue`](https://pkg.go.dev/k8s.io/apimachinery/pkg/api/resource#QuantityValue)|
[Alpha] InfoBufferSize sets the size of the info stream when
using split streams. The default is zero, which disables buffering.
Only available when the LoggingAlphaOptions feature gate is enabled.
|