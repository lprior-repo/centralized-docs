---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#60-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 87
summary: #### Note: The `stringData` field for a Secret does not work well with server-side apply. The basic authentication Secret type is provided only for convenience. You can create an `Opaque` type for...
---

#### Note:
The `stringData` field for a Secret does not work well with server-side apply.
The basic authentication Secret type is provided only for convenience.
You can create an `Opaque` type for credentials used for basic authentication.
However, using the defined and public Secret type (`kubernetes.io/basic-auth`) helps other
people to understand the purpose of your Secret, and sets a convention for what key names
to expect.