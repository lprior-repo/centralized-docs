---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#53-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 119
summary: #### Note: If you do not want to perform the base64 encoding, you can choose to use the `stringData` field instead. When you create Docker config Secrets using a manifest, the API server checks...
---

#### Note:
If you do not want to perform the base64 encoding, you can choose to use the
`stringData` field instead.
When you create Docker config Secrets using a manifest, the API
server checks whether the expected key exists in the `data` field, and
it verifies if the value provided can be parsed as a valid JSON. The API
server doesn't validate if the JSON actually is a Docker config file.
You can also use `kubectl` to create a Secret for accessing a container
registry, such as when you don't have a Docker configuration file: