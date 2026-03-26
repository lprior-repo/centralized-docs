---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#45-summary
chunk_level: summary
chunk_type: prose
heading: `KubeProxyConfiguration`
token_count: 121
summary: | detectLocalMode determines mode to use for detecting local traffic, defaults to ClusterCIDR | |`detectLocal`**[Required]**...
---

|
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