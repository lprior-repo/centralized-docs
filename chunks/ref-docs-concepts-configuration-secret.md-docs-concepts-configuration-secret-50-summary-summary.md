---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#50-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 102
summary: * `kubernetes.io/dockerconfigjson`: store a serialized JSON that follows the same format rules as the `\~/.docker/config.json` file, which is a new format for `\~/.dockercfg`. The Secret `data` field...
---

* `kubernetes.io/dockerconfigjson`: store a serialized JSON that follows the
same format rules as the `\~/.docker/config.json` file, which is a new format
for `\~/.dockercfg`. The Secret `data` field must contain a
`.dockerconfigjson` key for which the value is the content of a base64
encoded `\~/.docker/config.json` file.
Below is an example for a `kubernetes.io/dockercfg` type of Secret: