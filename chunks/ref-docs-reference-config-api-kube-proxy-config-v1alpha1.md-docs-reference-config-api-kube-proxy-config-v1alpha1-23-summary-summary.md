---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#23-summary
chunk_level: summary
chunk_type: table
heading: `ClientConnectionConfiguration`
token_count: 126
summary: * [GenericControllerManagerConfiguration](#controllermanager-config-k8s-io-v1alpha1-GenericControllerManagerConfiguration) ClientConnectionConfiguration contains details for constructing a client....
---

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
|