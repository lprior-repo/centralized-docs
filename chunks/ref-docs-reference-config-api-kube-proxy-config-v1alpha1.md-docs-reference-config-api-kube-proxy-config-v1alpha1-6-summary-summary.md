---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#6-summary
chunk_level: summary
chunk_type: table
heading: Resource Types
token_count: 110
summary: * [LoggingConfiguration](#LoggingConfiguration) FormatOptions contains options for the different logging formats. |Field|Description| |`text`**[Required]** [`TextOptions`](#TextOptions)| [Alpha] Text...
---

* [LoggingConfiguration](#LoggingConfiguration)
FormatOptions contains options for the different logging formats.
|Field|Description|
|`text`**[Required]**
[`TextOptions`](#TextOptions)|
[Alpha] Text contains options for logging format "text".
Only available when the LoggingAlphaOptions feature gate is enabled.
|
|`json`**[Required]**
[`JSONOptions`](#JSONOptions)|
[Alpha] JSON contains options for logging format "json".
Only available when the LoggingAlphaOptions feature gate is enabled.
|