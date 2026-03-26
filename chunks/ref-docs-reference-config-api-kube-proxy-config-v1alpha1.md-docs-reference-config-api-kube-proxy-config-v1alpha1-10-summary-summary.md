---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#10-summary
chunk_level: summary
chunk_type: table
heading: `LoggingConfiguration`
token_count: 112
summary: * [KubeletConfiguration](#kubelet-config-k8s-io-v1beta1-KubeletConfiguration) LoggingConfiguration contains logging options. |Field|Description| |`format`**[Required]** `string`| Format Flag...
---

* [KubeletConfiguration](#kubelet-config-k8s-io-v1beta1-KubeletConfiguration)
LoggingConfiguration contains logging options.
|Field|Description|
|`format`**[Required]**
`string`|
Format Flag specifies the structure of log messages.
default value of format is `text`
|
|`flushFrequency`**[Required]**
[`TimeOrMetaDuration`](#TimeOrMetaDuration)|
Maximum time between log flushes.
If a string, parsed as a duration (i.e. "1s")