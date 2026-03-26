---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#88-summary
chunk_level: summary
chunk_type: prose
heading: `WebhookConfiguration`
token_count: 127
summary: [`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)| Timeout for the webhook request Maximum allowed value is 30s. Required, no default value. |...
---

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