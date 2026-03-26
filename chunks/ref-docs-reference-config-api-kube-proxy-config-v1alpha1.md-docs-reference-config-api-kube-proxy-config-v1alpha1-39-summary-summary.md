---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#39-summary
chunk_level: summary
chunk_type: prose
heading: `KubeProxyConfiguration`
token_count: 125
summary: not actually bind any sockets to this IP. | |`healthzBindAddress`**[Required]** `string`| healthzBindAddress is the IP address and port for the health check server to serve on, defaulting to...
---

not actually bind any sockets to this IP.
|
|`healthzBindAddress`**[Required]**
`string`|
healthzBindAddress is the IP address and port for the health check server to
serve on, defaulting to "0.0.0.0:10256" (if bindAddress is unset or IPv4), or
"[::]:10256" (if bindAddress is IPv6).
|
|`metricsBindAddress`**[Required]**
`string`|
metricsBindAddress is the IP address and port for the metrics server to serve
on, defaulting to "