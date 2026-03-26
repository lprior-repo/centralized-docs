---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#85-summary
chunk_level: summary
chunk_type: table
heading: `WebhookConfiguration`
token_count: 123
summary: * [AuthorizerConfiguration](#apiserver-k8s-io-v1beta1-AuthorizerConfiguration)|Field|Description| |`authorizedTTL`**[Required]**...
---

* [AuthorizerConfiguration](#apiserver-k8s-io-v1beta1-AuthorizerConfiguration)|Field|Description|
|`authorizedTTL`**[Required]**
[`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|
The duration to cache 'authorized' responses from the webhook
authorizer.
Same as setting `--authorization-webhook-cache-authorized-ttl` flag
Default: 5m0s
|
|`cacheAuthorizedRequests`
`bool`|
CacheAuthorizedRequests specifies whether authorized requests should be cached.