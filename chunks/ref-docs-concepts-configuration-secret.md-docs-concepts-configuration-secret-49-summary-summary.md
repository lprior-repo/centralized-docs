---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#49-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 67
summary: * `kubernetes.io/dockercfg`: store a serialized `\~/.dockercfg` which is the legacy format for configuring Docker command line. The Secret `data` field contains a `.dockercfg` key whose value is the...
---

* `kubernetes.io/dockercfg`: store a serialized `\~/.dockercfg` which is the
legacy format for configuring Docker command line. The Secret
`data` field contains a `.dockercfg` key whose value is the content of a
base64 encoded `\~/.dockercfg` file.