---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#1-detailed
chunk_level: detailed
chunk_type: table
heading: `DebuggingConfiguration`
token_count: 1011
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
## `VModuleConfiguration`
(Alias of `[]k8s.io/component-base/logs/api/v1.VModuleItem`)
**Appears in:**
* [LoggingConfiguration](#LoggingConfiguration)
VModuleConfiguration is a collection of individual file names or patterns
and the corresponding verbosity threshold.
## `VerbosityLevel`
(Alias of `uint32`)
**Appears in:**
* [LoggingConfiguration](#LoggingConfiguration)
VerbosityLevel represents a klog or logr verbosity threshold.
## `ClientConnectionConfiguration`
**Appears in:**
* [KubeProxyConfiguration](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConfiguration)
* [KubeSchedulerConfiguration](#kubescheduler-config-k8s-io-v1-KubeSchedulerConfiguration)
* [GenericControllerManagerConfiguration](#controllermanager-config-k8s-io-v1alpha1-GenericControllerManagerConfiguration)
ClientConnectionConfiguration contains details for constructing a client.
|Field|Description|
|`kubeconfig`**[Required]**
`string`|
kubeconfig is the path to a KubeConfig file.
|
|`acceptContentTypes`**[Required]**
`string`|
acceptContentTypes defines the Accept header sent by clients when connecting to a server, overriding the
default value of 'application/json'. This field will control all connections to the server used by a particular
client.
|
|`contentType`**[Required]**
`string`|
contentType is the content type used when sending data to the server from this client.
|
|`qps`**[Required]**
`float32`|
qps controls the number of queries per second allowed for this connection.
|
|`burst`**[Required]**
`int32`|
burst allows extra queries to accumulate when a client is exceeding its rate.
|
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