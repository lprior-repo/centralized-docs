---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#7-standard
chunk_level: standard
chunk_type: table
heading: `KubeProxyConfiguration`
token_count: 508
summary: KubeProxyConfiguration contains everything necessary to configure the Kubernetes proxy server. |Field|Description| |`apiVersion` string|`kubeproxy.config.k8s.io/v1alpha1`| |`kind`...
---

KubeProxyConfiguration contains everything necessary to configure the
Kubernetes proxy server.
|Field|Description|
|`apiVersion`
string|`kubeproxy.config.k8s.io/v1alpha1`|
|`kind`
string|`KubeProxyConfiguration`|
|`featureGates`**[Required]**
`map[string]bool`|
featureGates is a map of feature names to bools that enable or disable alpha/experimental features.
|
|`clientConnection`**[Required]**
[`ClientConnectionConfiguration`](#ClientConnectionConfiguration)|
clientConnection specifies the kubeconfig file and client connection settings for the proxy
server to use when communicating with the apiserver.
|
|`logging`**[Required]**
[`LoggingConfiguration`](#LoggingConfiguration)|
logging specifies the options of logging.
Refer to [Logs Options](https://github.com/kubernetes/component-base/blob/master/logs/options.go)
for more information.
|
|`hostnameOverride`**[Required]**
`string`|
hostnameOverride, if non-empty, will be used as the name of the Node that
kube-proxy is running on. If unset, the node name is assumed to be the same as
the node's hostname.
|
|`bindAddress`**[Required]**
`string`|
bindAddress can be used to override kube-proxy's idea of what its node's
primary IP is. Note that the name is a historical artifact, and kube-proxy does
not actually bind any sockets to this IP.
|
|`healthzBindAddress`**[Required]**
`string`|
healthzBindAddress is the IP address and port for the health check server to
serve on, defaulting to "0.0.0.0:10256" (if bindAddress is unset or IPv4), or
"[::]:10256" (if bindAddress is IPv6).
|
|`metricsBindAddress`**[Required]**
`string`|
metricsBindAddress is the IP address and port for the metrics server to serve
on, defaulting to "127.0.0.1:10249" (if bindAddress is unset or IPv4), or
"[::1]:10249" (if bindAddress is IPv6). (Set to "0.0.0.0:10249" / "[::]:10249"
to bind on all interfaces.)
|
|`bindAddressHardFail`**[Required]**
`bool`|