---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#20-standard
chunk_level: standard
chunk_type: table
heading: `WebhookConfiguration`
token_count: 507
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
If set to true, the TTL for cached decisions can be configured via the
AuthorizedTTL field.
Default: true
|
|`unauthorizedTTL`**[Required]**
[`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|
The duration to cache 'unauthorized' responses from the webhook
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
Maximum allowed value is 30s.
Required, no default value.
|
|`subjectAccessReviewVersion`**[Required]**
`string`|
The API version of the authorization.k8s.io SubjectAccessReview to
send to and expect from the webhook.
Same as setting `--authorization-webhook-version` flag
Valid values: v1beta1, v1
Required, no default value
|
|`matchConditionSubjectAccessReviewVersion`**[Required]**
`string`|
MatchConditionSubjectAccessReviewVersion specifies the SubjectAccessReview
version the CEL expressions are evaluated against
Valid values: v1
Required, no default value
|
|`failurePolicy`**[Required]**
`string`|
Controls the authorization decision when a webhook request fails to
complete or returns a malformed response or errors evaluating
matchConditions.
Valid values:
* NoOpinion: continue to subsequent authorizers to see if one of
them allows the request