---
doc_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1
chunk_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1#7-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingWebhookConfiguration
token_count: 70
summary: * **webhooks** ([]ValidatingWebhook) *Patch strategy: merge on key `name`* *Map: unique values on key name will be kept during a merge* Webhooks is a list of webhooks and the affected resources and...
---

* **webhooks** ([]ValidatingWebhook)
*Patch strategy: merge on key `name`*
*Map: unique values on key name will be kept during a merge*
Webhooks is a list of webhooks and the affected resources and operations.
*ValidatingWebhook describes an admission webhook and the resources and operations it applies to.*