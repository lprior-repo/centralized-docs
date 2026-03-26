---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#2-standard
chunk_level: standard
chunk_type: table
heading: `TimeOrMetaDuration`
token_count: 492
summary: ## `LoggingOptions` LoggingOptions can be used with ValidateAndApplyWithOptions to override certain global defaults. |Field|Description| |`ErrorStream`**[Required]**...
---

## `LoggingOptions`
LoggingOptions can be used with ValidateAndApplyWithOptions to override
certain global defaults.
|Field|Description|
|`ErrorStream`**[Required]**
[`io.Writer`](https://pkg.go.dev/io#Writer)|
ErrorStream can be used to override the os.Stderr default.
|
|`InfoStream`**[Required]**
[`io.Writer`](https://pkg.go.dev/io#Writer)|
InfoStream can be used to override the os.Stdout default.
|
## `OutputRoutingOptions`
**Appears in:**
* [JSONOptions](#JSONOptions)
* [TextOptions](#TextOptions)
OutputRoutingOptions contains options that are supported by both "text" and "json".
|Field|Description|
|`splitStream`**[Required]**
`bool`|
[Alpha] SplitStream redirects error messages to stderr while
info messages go to stdout, with buffering. The default is to write
both to stdout, without buffering. Only available when
the LoggingAlphaOptions feature gate is enabled.
|
|`infoBufferSize`**[Required]**
[`k8s.io/apimachinery/pkg/api/resource.QuantityValue`](https://pkg.go.dev/k8s.io/apimachinery/pkg/api/resource#QuantityValue)|
[Alpha] InfoBufferSize sets the size of the info stream when
using split streams. The default is zero, which disables buffering.
Only available when the LoggingAlphaOptions feature gate is enabled.
|
## `TextOptions`
**Appears in:**
* [FormatOptions](#FormatOptions)
TextOptions contains options for logging format "text".
|Field|Description|
|`OutputRoutingOptions`**[Required]**
[`OutputRoutingOptions`](#OutputRoutingOptions)|(Members of `OutputRoutingOptions` are embedded into this type.)
No description provided.|
## `TimeOrMetaDuration`
**Appears in:**
* [LoggingConfiguration](#LoggingConfiguration)
TimeOrMetaDuration is present only for backwards compatibility for the
flushFrequency field, and new fields should use metav1.Duration.
|Field|Description|
|`Duration`**[Required]**
[`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|
Duration holds the duration
|
|`-`**[Required]**
`bool`|
SerializeAsString controls whether the value is serialized as a string or an integer
|