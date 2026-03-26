---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#91-summary
chunk_level: summary
chunk_type: prose
heading: `WebhookConfiguration`
token_count: 119
summary: * Deny: reject the request without consulting subsequent authorizers Required, with no default.| |`connectionInfo`**[Required]**...
---

* Deny: reject the request without consulting subsequent authorizers
Required, with no default.|
|`connectionInfo`**[Required]**
[`WebhookConnectionInfo`](#apiserver-k8s-io-v1beta1-WebhookConnectionInfo)|
ConnectionInfo defines how we talk to the webhook
|
|`matchConditions`**[Required]**
[`[]WebhookMatchCondition`](#apiserver-k8s-io-v1beta1-WebhookMatchCondition)|
matchConditions is a list of conditions that must be met for a request to be sent to this