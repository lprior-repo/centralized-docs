---
doc_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1
chunk_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1#10-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingWebhookConfiguration
token_count: 119
summary: * **webhooks.clientConfig.service** (ServiceReference) `service` is a reference to the service for this webhook. Either `service` or `url` must be specified. If the webhook is running within the...
---

* **webhooks.clientConfig.service** (ServiceReference)
`service` is a reference to the service for this webhook. Either `service` or `url` must be specified.
If the webhook is running within the cluster, then you should use `service`.
*ServiceReference holds a reference to Service.legacy.k8s.io*
* **webhooks.clientConfig.service.name** (string), required
`name` is the name of the service. Required
* **webhooks.clientConfig.service.namespace** (string), required
`namespace` is the namespace of the service. Required