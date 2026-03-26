---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#1-standard
chunk_level: standard
chunk_type: table
heading: Resource Types
token_count: 317
summary: # kube-apiserver Configuration (v1beta1) Package v1beta1 is the v1beta1 version of the API. ## Resource Types * [AuthenticationConfiguration](#apiserver-k8s-io-v1beta1-AuthenticationConfiguration) *...
---

# kube-apiserver Configuration (v1beta1)
Package v1beta1 is the v1beta1 version of the API.
## Resource Types
* [AuthenticationConfiguration](#apiserver-k8s-io-v1beta1-AuthenticationConfiguration)
* [AuthorizationConfiguration](#apiserver-k8s-io-v1beta1-AuthorizationConfiguration)
* [EgressSelectorConfiguration](#apiserver-k8s-io-v1beta1-EgressSelectorConfiguration)
* [TracingConfiguration](#apiserver-k8s-io-v1beta1-TracingConfiguration)## `TracingConfiguration`
**Appears in:**
* [KubeletConfiguration](#kubelet-config-k8s-io-v1beta1-KubeletConfiguration)
* [TracingConfiguration](#apiserver-k8s-io-v1alpha1-TracingConfiguration)
* [TracingConfiguration](#apiserver-k8s-io-v1beta1-TracingConfiguration)
TracingConfiguration provides versioned configuration for OpenTelemetry tracing clients.
|Field|Description|
|`endpoint`
`string`|
Endpoint of the collector this component will report traces to.
The connection is insecure, and does not currently support TLS.
Recommended is unset, and endpoint is the otlp grpc default, localhost:4317.
|
|`samplingRatePerMillion`
`int32`|
SamplingRatePerMillion is the number of samples to collect per million spans.
Recommended is unset. If unset, sampler respects its parent span's sampling
rate, but otherwise never samples.
|