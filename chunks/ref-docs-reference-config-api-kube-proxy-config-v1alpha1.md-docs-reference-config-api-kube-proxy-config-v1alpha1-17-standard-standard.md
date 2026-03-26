---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#17-standard
chunk_level: standard
chunk_type: table
heading: `ProxyMode`
token_count: 467
summary: ## `KubeProxyWinkernelConfiguration` **Appears in:** * [KubeProxyConfiguration](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConfiguration) KubeProxyWinkernelConfiguration contains Windows/HNS settings...
---

## `KubeProxyWinkernelConfiguration`
**Appears in:**
* [KubeProxyConfiguration](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConfiguration)
KubeProxyWinkernelConfiguration contains Windows/HNS settings for
the Kubernetes proxy server.
|Field|Description|
|`networkName`**[Required]**
`string`|
networkName is the name of the network kube-proxy will use
to create endpoints and policies
|
|`sourceVip`**[Required]**
`string`|
sourceVip is the IP address of the source VIP endpoint used for
NAT when loadbalancing
|
|`enableDSR`**[Required]**
`bool`|
enableDSR tells kube-proxy whether HNS policies should be created
with DSR
|
|`rootHnsEndpointName`**[Required]**
`string`|
rootHnsEndpointName is the name of hnsendpoint that is attached to
l2bridge for root network namespace
|
|`forwardHealthCheckVip`**[Required]**
`bool`|
forwardHealthCheckVip forwards service VIP for health check port on
Windows
|
## `LocalMode`
(Alias of `string`)
**Appears in:**
* [KubeProxyConfiguration](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConfiguration)
LocalMode represents modes to detect local traffic from the node
## `ProxyMode`
(Alias of `string`)
**Appears in:**
* [KubeProxyConfiguration](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConfiguration)
ProxyMode represents modes used by the Kubernetes proxy server.
Three modes of proxy are available on Linux platforms: `iptables`, `ipvs`, and
`nftables`. One mode of proxy is available on Windows platforms: `kernelspace`.
If the proxy mode is unspecified, a default proxy mode will be used (currently this
is `iptables` on Linux and `kernelspace` on Windows). If the selected proxy mode cannot be
used (due to lack of kernel support, missing userspace components, etc) then kube-proxy
will exit with an error.