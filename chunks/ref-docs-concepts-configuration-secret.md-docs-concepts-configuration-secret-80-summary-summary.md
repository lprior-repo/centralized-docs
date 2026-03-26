---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#80-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 128
summary: * `usage-bootstrap-&lt;usage&gt;`: A boolean flag indicating additional usage for the bootstrap token. * `auth-extra-groups`: A comma-separated list of group names that will be authenticated as in...
---

* `usage-bootstrap-&lt;usage&gt;`: A boolean flag indicating additional usage for
the bootstrap token.
* `auth-extra-groups`: A comma-separated list of group names that will be
authenticated as in addition to the `system:bootstrappers` group.
You can alternatively provide the values in the `stringData` field of the Secret
without base64 encoding them:
[`secret/bootstrap-token-secret-literal.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/secret/bootstrap-token-secret-literal.yaml)![](/images/copycode.svg "Copy secret/bootstrap-token-secret-literal.yaml to clipboard")