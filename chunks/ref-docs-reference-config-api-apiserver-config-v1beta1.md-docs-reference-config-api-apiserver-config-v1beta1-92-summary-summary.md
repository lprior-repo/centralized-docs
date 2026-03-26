---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#92-summary
chunk_level: summary
chunk_type: prose
heading: `WebhookConfiguration`
token_count: 106
summary: | matchConditions is a list of conditions that must be met for a request to be sent to this webhook. An empty list of matchConditions matches all requests. There are a maximum of 64 match conditions...
---

|
matchConditions is a list of conditions that must be met for a request to be sent to this
webhook. An empty list of matchConditions matches all requests.
There are a maximum of 64 match conditions allowed.
The exact matching logic is (in order):
1. If at least one matchCondition evaluates to FALSE, then the webhook is skipped.
2. If ALL matchConditions evaluate to TRUE, then the webhook is called.
3. If at least one matchCondition evaluates to an error (but none are FALSE):