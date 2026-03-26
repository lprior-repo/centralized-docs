---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#87-summary
chunk_level: summary
chunk_type: prose
heading: `WebhookConfiguration`
token_count: 120
summary: 'unauthorized' responses from the webhook authorizer. Same as setting `--authorization-webhook-cache-unauthorized-ttl` flag Default: 30s | |`cacheUnauthorizedRequests` `bool`|...
---

'unauthorized' responses from the webhook
authorizer.
Same as setting `--authorization-webhook-cache-unauthorized-ttl` flag
Default: 30s
|
|`cacheUnauthorizedRequests`
`bool`|
CacheUnauthorizedRequests specifies whether unauthorized requests should be cached.
If set to true, the TTL for cached decisions can be configured via the
UnauthorizedTTL field.
Default: true
|
|`timeout`**[Required]**
[`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|
Timeout for the webhook request