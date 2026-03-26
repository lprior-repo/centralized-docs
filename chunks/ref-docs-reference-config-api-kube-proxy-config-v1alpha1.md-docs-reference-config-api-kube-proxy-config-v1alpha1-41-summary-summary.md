---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#41-summary
chunk_level: summary
chunk_type: prose
heading: `KubeProxyConfiguration`
token_count: 120
summary: **[Required]** `bool`| bindAddressHardFail, if true, tells kube-proxy to treat failure to bind to a port as fatal and exit | |`enableProfiling`**[Required]** `bool`| enableProfiling enables profiling...
---

**[Required]**
`bool`|
bindAddressHardFail, if true, tells kube-proxy to treat failure to bind to a
port as fatal and exit
|
|`enableProfiling`**[Required]**
`bool`|
enableProfiling enables profiling via web interface on /debug/pprof handler.
Profiling handlers will be handled by metrics server.
|
|`showHiddenMetricsForVersion`**[Required]**
`string`|
showHiddenMetricsForVersion is the version for which you want to show hidden metrics.
|
|`mode`**[Required]**