---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#9-detailed
chunk_level: detailed
chunk_type: table
heading: `WebhookConnectionInfo`
token_count: 920
summary: ## `WebhookConfiguration` **Appears in:** * [AuthorizerConfiguration](#apiserver-k8s-io-v1beta1-AuthorizerConfiguration)|Field|Description| |`authorizedTTL`**[Required]**...
---

## `WebhookConfiguration`
**Appears in:**
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