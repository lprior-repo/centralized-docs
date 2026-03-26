---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#71-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 118
summary: The TLS Secret type is provided only for convenience. You can create an `Opaque` type for credentials used for TLS authentication. However, using the defined and public Secret type...
---

The TLS Secret type is provided only for convenience.
You can create an `Opaque` type for credentials used for TLS authentication.
However, using the defined and public Secret type (`kubernetes.io/tls`)
helps ensure the consistency of Secret format in your project. The API server
verifies if the required keys are set for a Secret of this type.
To create a TLS Secret using `kubectl`, use the `tls` subcommand:
```
`kubectl create secret tls my-tls-secret \\
--cert=path/to/cert/file \\
--key=path/to/key/file
`
```