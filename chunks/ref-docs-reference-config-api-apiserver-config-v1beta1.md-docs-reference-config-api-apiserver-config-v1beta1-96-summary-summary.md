---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#96-summary
chunk_level: summary
chunk_type: prose
heading: `WebhookConnectionInfo`
token_count: 95
summary: * KubeConfigFile: use the file specified in kubeConfigFile to locate the server. * InClusterConfig: use the in-cluster configuration to call the SubjectAccessReview API hosted by kube-apiserver. This...
---

* KubeConfigFile: use the file specified in kubeConfigFile to locate the
server.
* InClusterConfig: use the in-cluster configuration to call the
SubjectAccessReview API hosted by kube-apiserver. This mode is not
allowed for kube-apiserver.|
|`kubeConfigFile`**[Required]**
`string`|
Path to KubeConfigFile for connection info
Required, if connectionInfo.Type is KubeConfig
|