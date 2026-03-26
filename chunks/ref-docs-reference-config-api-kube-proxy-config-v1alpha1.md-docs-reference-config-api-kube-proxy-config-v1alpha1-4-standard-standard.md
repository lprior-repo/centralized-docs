---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#4-standard
chunk_level: standard
chunk_type: table
heading: `DebuggingConfiguration`
token_count: 143
summary: ## `DebuggingConfiguration` **Appears in:** * [KubeSchedulerConfiguration](#kubescheduler-config-k8s-io-v1-KubeSchedulerConfiguration) *...
---

## `DebuggingConfiguration`
**Appears in:**
* [KubeSchedulerConfiguration](#kubescheduler-config-k8s-io-v1-KubeSchedulerConfiguration)
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