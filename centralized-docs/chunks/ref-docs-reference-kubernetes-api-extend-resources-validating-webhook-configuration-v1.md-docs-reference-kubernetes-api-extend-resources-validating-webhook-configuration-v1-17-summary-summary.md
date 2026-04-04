---
doc_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1
chunk_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1#17-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingWebhookConfiguration
token_count: 89
summary: * `\"None\"` means that calling the webhook will have no side effects. * `\"NoneOnDryRun\"` means that calling the webhook will possibly have side effects, but if the request being reviewed has the...
---

* `"None"` means that calling the webhook will have no side effects.
* `"NoneOnDryRun"` means that calling the webhook will possibly have side effects, but if the request being reviewed has the dry-run attribute, the side effects will be suppressed.
* `"Some"` means that calling the webhook will possibly have side effects. If a request with the dry-run attribute would trigger a call to this webhook, the request will instead fail.