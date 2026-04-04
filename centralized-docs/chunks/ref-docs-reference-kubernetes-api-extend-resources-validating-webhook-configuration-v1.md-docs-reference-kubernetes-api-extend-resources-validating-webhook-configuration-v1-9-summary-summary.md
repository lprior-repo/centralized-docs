---
doc_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1
chunk_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1#9-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingWebhookConfiguration
token_count: 91
summary: * **webhooks.clientConfig** (WebhookClientConfig), required ClientConfig defines how to communicate with the hook. Required *WebhookClientConfig contains the information to make a TLS connection with...
---

* **webhooks.clientConfig** (WebhookClientConfig), required
ClientConfig defines how to communicate with the hook. Required
*WebhookClientConfig contains the information to make a TLS connection with the webhook*
* **webhooks.clientConfig.caBundle** ([]byte)
`caBundle` is a PEM encoded CA bundle which will be used to validate the webhook's server certificate. If unspecified, system trust roots on the apiserver are used.