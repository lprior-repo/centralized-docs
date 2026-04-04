---
doc_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1
chunk_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1#26-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingWebhookConfiguration
token_count: 70
summary: .]\*)?[A-Za-z0-9]') with an optional DNS subdomain prefix and '/' (e.g. 'example.com/MyName') Required. * **webhooks.matchPolicy** (string) matchPolicy defines how the \"rules\" list is used to match...
---

.]\*)?[A-Za-z0-9]') with an optional DNS subdomain prefix and '/' (e.g. 'example.com/MyName')
Required.
* **webhooks.matchPolicy** (string)
matchPolicy defines how the "rules" list is used to match incoming requests. Allowed values are "Exact" or "Equivalent".