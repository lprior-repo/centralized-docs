---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#5-detailed
chunk_level: detailed
chunk_type: table
heading: `DetectLocalConfiguration`
token_count: 750
summary: [`KubeProxyWinkernelConfiguration`](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyWinkernelConfiguration)| winkernel contains winkernel-related configuration options. | |`detectLocalMode`**[Required]**...
---

[`KubeProxyWinkernelConfiguration`](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyWinkernelConfiguration)|
winkernel contains winkernel-related configuration options.
|
|`detectLocalMode`**[Required]**
[`LocalMode`](#kubeproxy-config-k8s-io-v1alpha1-LocalMode)|
detectLocalMode determines mode to use for detecting local traffic, defaults to ClusterCIDR
|
|`detectLocal`**[Required]**
[`DetectLocalConfiguration`](#kubeproxy-config-k8s-io-v1alpha1-DetectLocalConfiguration)|
detectLocal contains optional configuration settings related to DetectLocalMode.
|
|`clusterCIDR`**[Required]**
`string`|
clusterCIDR is the CIDR range of the pods in the cluster. (For dual-stack
clusters, this can be a comma-separated dual-stack pair of CIDR ranges.). When
DetectLocalMode is set to ClusterCIDR, kube-proxy will consider
traffic to be local if its source IP is in this range. (Otherwise it is not
used.)
|
|`nodePortAddresses`**[Required]**
`[]string`|
nodePortAddresses is a list of CIDR ranges that contain valid node IPs, or
alternatively, the single string 'primary'. If set to a list of CIDRs,
connections to NodePort services will only be accepted on node IPs in one of
the indicated ranges. If set to 'primary', NodePort services will only be
accepted on the node's primary IPv4 and/or IPv6 address according to the Node
object. If unset, NodePort connections will be accepted on all local IPs.
|
|`oomScoreAdj`**[Required]**
`int32`|
oomScoreAdj is the oom-score-adj value for kube-proxy process. Values must be within
the range [-1000, 1000]
|
|`conntrack`**[Required]**
[`KubeProxyConntrackConfiguration`](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConntrackConfiguration)|
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