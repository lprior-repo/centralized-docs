---
doc_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1
chunk_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1#20-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingWebhookConfiguration
token_count: 46
summary: 1. If ANY matchCondition evaluates to FALSE, the webhook is skipped. 2. If ALL matchConditions evaluate to TRUE, the webhook is called. 3. If any matchCondition evaluates to an error (but none are...
---

1. If ANY matchCondition evaluates to FALSE, the webhook is skipped.
2. If ALL matchConditions evaluate to TRUE, the webhook is called.
3. If any matchCondition evaluates to an error (but none are FALSE):