---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#49-summary
chunk_level: summary
chunk_type: prose
heading: `KubeProxyConfiguration`
token_count: 89
summary: | configSyncPeriod is how often configuration from the apiserver is refreshed. Must be greater than 0. | |`portRange`**[Required]** `string`| portRange was previously used to configure the userspace...
---

|
configSyncPeriod is how often configuration from the apiserver is refreshed. Must be greater
than 0.
|
|`portRange`**[Required]**
`string`|
portRange was previously used to configure the userspace proxy, but is now unused.
|
|`windowsRunAsService`**[Required]**
`bool`|
windowsRunAsService, if true, enables Windows service control manager API integration.
|