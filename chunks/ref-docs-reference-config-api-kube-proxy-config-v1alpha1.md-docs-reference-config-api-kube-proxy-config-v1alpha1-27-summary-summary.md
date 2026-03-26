---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#27-summary
chunk_level: summary
chunk_type: table
heading: `DebuggingConfiguration`
token_count: 108
summary: * [GenericControllerManagerConfiguration](#controllermanager-config-k8s-io-v1alpha1-GenericControllerManagerConfiguration) DebuggingConfiguration holds configuration for Debugging related features....
---

* [GenericControllerManagerConfiguration](#controllermanager-config-k8s-io-v1alpha1-GenericControllerManagerConfiguration)
DebuggingConfiguration holds configuration for Debugging related features.
|Field|Description|
|`enableProfiling`**[Required]**
`bool`|
enableProfiling enables profiling via web interface host:port/debug/pprof/
|
|`enableContentionProfiling`**[Required]**
`bool`|
enableContentionProfiling enables block profiling, if
enableProfiling is true.
|