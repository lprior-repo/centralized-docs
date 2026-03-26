---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#89-summary
chunk_level: summary
chunk_type: prose
heading: `WebhookConfiguration`
token_count: 109
summary: Valid values: v1beta1, v1 Required, no default value | |`matchConditionSubjectAccessReviewVersion`**[Required]** `string`| MatchConditionSubjectAccessReviewVersion specifies the SubjectAccessReview...
---

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