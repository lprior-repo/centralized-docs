---
doc_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1
chunk_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1#19-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingWebhookConfiguration
token_count: 122
summary: * `\"Ignore\"` means that an error calling the webhook is ignored. * **webhooks.matchConditions** ([]MatchCondition) *Patch strategy: merge on key `name`* *Map: unique values on key name will be kept...
---

* `"Ignore"` means that an error calling the webhook is ignored.
* **webhooks.matchConditions** ([]MatchCondition)
*Patch strategy: merge on key `name`*
*Map: unique values on key name will be kept during a merge*
MatchConditions is a list of conditions that must be met for a request to be sent to this webhook. Match conditions filter requests that have already been matched by the rules, namespaceSelector, and objectSelector. An empty list of matchConditions matches all requests. There are a maximum of 64 match conditions allowed.
The exact matching logic is (in order):