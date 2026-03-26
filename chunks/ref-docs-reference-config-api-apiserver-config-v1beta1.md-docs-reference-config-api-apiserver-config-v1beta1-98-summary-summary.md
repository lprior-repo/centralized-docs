---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#98-summary
chunk_level: summary
chunk_type: table
heading: `WebhookMatchCondition`
token_count: 106
summary: * [WebhookConfiguration](#apiserver-k8s-io-v1beta1-WebhookConfiguration)|Field|Description| |`expression`**[Required]** `string`| expression represents the expression which will be evaluated by CEL....
---

* [WebhookConfiguration](#apiserver-k8s-io-v1beta1-WebhookConfiguration)|Field|Description|
|`expression`**[Required]**
`string`|
expression represents the expression which will be evaluated by CEL. Must evaluate to bool.
CEL expressions have access to the contents of the SubjectAccessReview in v1 version.
If version specified by subjectAccessReviewVersion in the request variable is v1beta1,
the contents would be converted to the v1 version before evaluating the CEL expression.