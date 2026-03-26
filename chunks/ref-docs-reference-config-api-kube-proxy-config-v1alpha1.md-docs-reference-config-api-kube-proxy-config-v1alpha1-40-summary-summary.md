---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#40-summary
chunk_level: summary
chunk_type: prose
heading: `KubeProxyConfiguration`
token_count: 127
summary: `string`| metricsBindAddress is the IP address and port for the metrics server to serve on, defaulting to \"127.0.0.1:10249\" (if bindAddress is unset or IPv4), or \"[::1]:10249\" (if bindAddress is...
---

`string`|
metricsBindAddress is the IP address and port for the metrics server to serve
on, defaulting to "127.0.0.1:10249" (if bindAddress is unset or IPv4), or
"[::1]:10249" (if bindAddress is IPv6). (Set to "0.0.0.0:10249" / "[::]:10249"
to bind on all interfaces.)
|
|`bindAddressHardFail`**[Required]**
`bool`|
bindAddressHardFail, if true, tells kube-proxy to treat failure to bind to a