---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#12-summary
chunk_level: summary
chunk_type: prose
heading: `LoggingConfiguration`
token_count: 123
summary: messages. Higher values enable additional messages. Error messages are always logged. | |`vmodule`**[Required]** [`VModuleConfiguration`](#VModuleConfiguration)| VModule overrides the verbosity...
---

messages. Higher values enable additional messages. Error messages
are always logged.
|
|`vmodule`**[Required]**
[`VModuleConfiguration`](#VModuleConfiguration)|
VModule overrides the verbosity threshold for individual files.
Only supported for "text" log format.
|
|`options`**[Required]**
[`FormatOptions`](#FormatOptions)|
[Alpha] Options holds additional parameters that are specific
to the different logging formats. Only the options for the selected
format get used, but all of them get validated.
Only available when the LoggingAlphaOptions feature gate is enabled.
|