---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#10-summary
chunk_level: summary
chunk_type: table
heading: Resource Types
token_count: 120
summary: * [TracingConfiguration](#apiserver-k8s-io-v1beta1-TracingConfiguration) TracingConfiguration provides versioned configuration for OpenTelemetry tracing clients. |Field|Description| |`endpoint`...
---

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