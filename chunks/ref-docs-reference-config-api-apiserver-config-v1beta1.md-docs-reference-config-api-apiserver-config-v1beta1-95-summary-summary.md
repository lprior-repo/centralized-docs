---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#95-summary
chunk_level: summary
chunk_type: table
heading: `WebhookConnectionInfo`
token_count: 72
summary: * [WebhookConfiguration](#apiserver-k8s-io-v1beta1-WebhookConfiguration)|Field|Description| |`type`**[Required]** `string`| Controls how the webhook should communicate with the server. Valid values:...
---

* [WebhookConfiguration](#apiserver-k8s-io-v1beta1-WebhookConfiguration)|Field|Description|
|`type`**[Required]**
`string`|
Controls how the webhook should communicate with the server.
Valid values:
* KubeConfigFile: use the file specified in kubeConfigFile to locate the
server.