---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#1-standard
chunk_level: standard
chunk_type: table
heading: `LoggingConfiguration`
token_count: 450
summary: ## `JSONOptions` **Appears in:** * [FormatOptions](#FormatOptions) JSONOptions contains options for logging format \"json\". |Field|Description| |`OutputRoutingOptions`**[Required]**...
---

## `JSONOptions`
**Appears in:**
* [FormatOptions](#FormatOptions)
JSONOptions contains options for logging format "json".
|Field|Description|
|`OutputRoutingOptions`**[Required]**
[`OutputRoutingOptions`](#OutputRoutingOptions)|(Members of `OutputRoutingOptions` are embedded into this type.)
No description provided.|
## `LogFormatFactory`
LogFormatFactory provides support for a certain additional,
non-default log format.
## `LoggingConfiguration`
**Appears in:**
* [KubeProxyConfiguration](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConfiguration)
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
If an int, the maximum number of nanoseconds (i.e. 1s = 1000000000).
Ignored if the selected logging backend writes log messages without buffering.
|
|`verbosity`**[Required]**
[`VerbosityLevel`](#VerbosityLevel)|
Verbosity is the threshold that determines which log messages are
logged. Default is zero which logs only the most important
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