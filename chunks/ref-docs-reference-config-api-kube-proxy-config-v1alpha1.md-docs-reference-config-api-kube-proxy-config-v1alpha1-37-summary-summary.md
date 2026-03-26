---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#37-summary
chunk_level: summary
chunk_type: prose
heading: `KubeProxyConfiguration`
token_count: 128
summary: | |`clientConnection`**[Required]** [`ClientConnectionConfiguration`](#ClientConnectionConfiguration)| clientConnection specifies the kubeconfig file and client connection settings for the proxy...
---

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