---
doc_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1
chunk_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1#11-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingWebhookConfiguration
token_count: 107
summary: * **webhooks.clientConfig.service.namespace** (string), required `namespace` is the namespace of the service. Required * **webhooks.clientConfig.service.path** (string) `path` is an optional URL path...
---

* **webhooks.clientConfig.service.namespace** (string), required
`namespace` is the namespace of the service. Required
* **webhooks.clientConfig.service.path** (string)
`path` is an optional URL path which will be sent in any request to this service.
* **webhooks.clientConfig.service.port** (int32)
If specified, the port on the service that hosting webhook. Default to 443 for backward compatibility. `port` should be a valid port number (1-65535, inclusive).