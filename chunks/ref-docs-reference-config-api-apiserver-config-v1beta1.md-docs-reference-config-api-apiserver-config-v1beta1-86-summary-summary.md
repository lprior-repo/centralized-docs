---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#86-summary
chunk_level: summary
chunk_type: prose
heading: `WebhookConfiguration`
token_count: 124
summary: Default: 5m0s | |`cacheAuthorizedRequests` `bool`| CacheAuthorizedRequests specifies whether authorized requests should be cached. If set to true, the TTL for cached decisions can be configured via...
---

Default: 5m0s
|
|`cacheAuthorizedRequests`
`bool`|
CacheAuthorizedRequests specifies whether authorized requests should be cached.
If set to true, the TTL for cached decisions can be configured via the
AuthorizedTTL field.
Default: true
|
|`unauthorizedTTL`**[Required]**
[`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|
The duration to cache 'unauthorized' responses from the webhook
authorizer.
Same as setting `--authorization-webhook-cache-unauthorized-ttl` flag