---
doc_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1
chunk_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1#16-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingWebhookConfiguration
token_count: 127
summary: * **webhooks.sideEffects** (string), required SideEffects states whether this webhook has side effects. Acceptable values are: None, NoneOnDryRun (webhooks created via v1beta1 may also specify Some...
---

* **webhooks.sideEffects** (string), required
SideEffects states whether this webhook has side effects. Acceptable values are: None, NoneOnDryRun (webhooks created via v1beta1 may also specify Some or Unknown). Webhooks with side effects MUST implement a reconciliation system, since a request may be rejected by a future step in the admission chain and the side effects therefore need to be undone. Requests with the dryRun attribute will be auto-rejected if they match a webhook with sideEffects == Unknown or Some.
Possible enum values:
* `"None"` means that calling the webhook will have no side effects.