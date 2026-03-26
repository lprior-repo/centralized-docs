---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#21-standard
chunk_level: standard
chunk_type: table
heading: `WebhookConnectionInfo`
token_count: 421
summary: * NoOpinion: continue to subsequent authorizers to see if one of them allows the request * Deny: reject the request without consulting subsequent authorizers Required, with no default.|...
---

* NoOpinion: continue to subsequent authorizers to see if one of
them allows the request
* Deny: reject the request without consulting subsequent authorizers
Required, with no default.|
|`connectionInfo`**[Required]**
[`WebhookConnectionInfo`](#apiserver-k8s-io-v1beta1-WebhookConnectionInfo)|
ConnectionInfo defines how we talk to the webhook
|
|`matchConditions`**[Required]**
[`[]WebhookMatchCondition`](#apiserver-k8s-io-v1beta1-WebhookMatchCondition)|
matchConditions is a list of conditions that must be met for a request to be sent to this
webhook. An empty list of matchConditions matches all requests.
There are a maximum of 64 match conditions allowed.
The exact matching logic is (in order):
1. If at least one matchCondition evaluates to FALSE, then the webhook is skipped.
2. If ALL matchConditions evaluate to TRUE, then the webhook is called.
3. If at least one matchCondition evaluates to an error (but none are FALSE):
* If failurePolicy=Deny, then the webhook rejects the request
* If failurePolicy=NoOpinion, then the error is ignored and the webhook is skipped|
## `WebhookConnectionInfo`
**Appears in:**
* [WebhookConfiguration](#apiserver-k8s-io-v1beta1-WebhookConfiguration)|Field|Description|
|`type`**[Required]**
`string`|
Controls how the webhook should communicate with the server.
Valid values:
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