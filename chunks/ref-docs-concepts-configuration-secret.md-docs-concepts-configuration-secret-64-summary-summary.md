---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#64-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 84
summary: The SSH authentication Secret type is provided only for convenience. You can create an `Opaque` type for credentials used for SSH authentication. However, using the defined and public Secret type...
---

The SSH authentication Secret type is provided only for convenience.
You can create an `Opaque` type for credentials used for SSH authentication.
However, using the defined and public Secret type (`kubernetes.io/ssh-auth`) helps other
people to understand the purpose of your Secret, and sets a convention for what key names
to expect.
The Kubernetes API verifies that the required keys are set for a Secret of this type.