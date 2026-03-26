---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#81-summary
chunk_level: summary
chunk_type: prose
heading: `ProxyMode`
token_count: 117
summary: * [KubeProxyConfiguration](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConfiguration) ProxyMode represents modes used by the Kubernetes proxy server. Three modes of proxy are available on Linux...
---

* [KubeProxyConfiguration](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConfiguration)
ProxyMode represents modes used by the Kubernetes proxy server.
Three modes of proxy are available on Linux platforms: `iptables`, `ipvs`, and
`nftables`. One mode of proxy is available on Windows platforms: `kernelspace`.
If the proxy mode is unspecified, a default proxy mode will be used (currently this
is `iptables` on Linux and `kernelspace` on Windows). If the selected proxy mode cannot be