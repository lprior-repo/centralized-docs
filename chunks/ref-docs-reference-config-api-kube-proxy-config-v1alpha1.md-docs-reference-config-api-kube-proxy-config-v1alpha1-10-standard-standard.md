---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#10-standard
chunk_level: standard
chunk_type: table
heading: `DetectLocalConfiguration`
token_count: 305
summary: | conntrack contains conntrack-related configuration options. | |`configSyncPeriod`**[Required]** [`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|...
---

|
conntrack contains conntrack-related configuration options.
|
|`configSyncPeriod`**[Required]**
[`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|
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
## `DetectLocalConfiguration`
**Appears in:**
* [KubeProxyConfiguration](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConfiguration)
DetectLocalConfiguration contains optional settings related to DetectLocalMode option
|Field|Description|
|`bridgeInterface`**[Required]**
`string`|
bridgeInterface is a bridge interface name. When DetectLocalMode is set to
LocalModeBridgeInterface, kube-proxy will consider traffic to be local if
it originates from this bridge.
|
|`interfaceNamePrefix`**[Required]**
`string`|
interfaceNamePrefix is an interface name prefix. When DetectLocalMode is set to
LocalModeInterfaceNamePrefix, kube-proxy will consider traffic to be local if
it originates from any interface whose name begins with this prefix.
|