---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#3-standard
chunk_level: standard
chunk_type: table
heading: `ClientConnectionConfiguration`
token_count: 376
summary: ## `VModuleConfiguration` (Alias of `[]k8s.io/component-base/logs/api/v1.VModuleItem`) **Appears in:** * [LoggingConfiguration](#LoggingConfiguration) VModuleConfiguration is a collection of...
---

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