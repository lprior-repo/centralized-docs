---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#83-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 74
summary: token-secret: \"kq4gihvszzgn1p0r\" # This token can be used for authentication usage-bootstrap-authentication: \"true\" # and it can be used for signing usage-bootstrap-signing: \"true\"` ``` #### Note:...
---

token-secret: "kq4gihvszzgn1p0r"
# This token can be used for authentication
usage-bootstrap-authentication: "true"
# and it can be used for signing
usage-bootstrap-signing: "true"`
```
#### Note:
The `stringData` field for a Secret does not work well with server-side apply.