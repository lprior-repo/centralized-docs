---
doc_id: ref/docs-concepts-security-controlling-access.md/docs-concepts-security-controlling-access
chunk_id: ref/docs-concepts-security-controlling-access.md/docs-concepts-security-controlling-access#6-summary
chunk_level: summary
chunk_type: prose
heading: Authentication
token_count: 123
summary: Once TLS is established, the HTTP request moves to the Authentication step. This is shown as step **1** in the diagram. The cluster creation script or cluster admin configures the API server to run...
---

Once TLS is established, the HTTP request moves to the Authentication step.
This is shown as step **1** in the diagram.
The cluster creation script or cluster admin configures the API server to run
one or more Authenticator modules.
Authenticators are described in more detail in
[Authentication](/docs/reference/access-authn-authz/authentication/).
The input to the authentication step is the entire HTTP request; however, it typically
examines the headers and/or client certificate.
Authentication modules include client certificates, password, and plain tokens,
bootstrap tokens, and JSON Web Tokens (used for service accounts).