---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#68-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 125
summary: key must be provided in the `data` (or `stringData`) field of the Secret configuration, although the API server doesn't actually validate the values for each key. As an alternative to using...
---

 key must be provided
in the `data` (or `stringData`) field of the Secret configuration, although the API
server doesn't actually validate the values for each key.
As an alternative to using `stringData`, you can use the `data` field to provide
the base64 encoded certificate and private key. For details, see
[Constraints on Secret names and data](#restriction-names-data).
The following YAML contains an example config for a TLS Secret:
[`secret/tls-auth-secret.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/secret/tls-auth-secret.yaml)