---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#24-summary
chunk_level: summary
chunk_type: prose
heading: `AuthorizerConfiguration`
token_count: 109
summary: This is explicitly used in monitoring machinery for metrics Note: Names must be DNS1123 labels like `myauthorizername` or subdomains like `myauthorizer.example.domain` Required, with no default |...
---

This is explicitly used in monitoring machinery for metrics
Note: Names must be DNS1123 labels like `myauthorizername` or
subdomains like `myauthorizer.example.domain`
Required, with no default
|
|`webhook`**[Required]**
[`WebhookConfiguration`](#apiserver-k8s-io-v1beta1-WebhookConfiguration)|
Webhook defines the configuration for a Webhook authorizer
Must be defined when Type=Webhook
Must not be defined when Type!=Webhook
|