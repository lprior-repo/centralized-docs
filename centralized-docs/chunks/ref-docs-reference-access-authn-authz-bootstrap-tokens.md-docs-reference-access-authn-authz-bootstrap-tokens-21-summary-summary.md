---
doc_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens
chunk_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens#21-summary
chunk_level: summary
chunk_type: prose
heading: ConfigMap Signing
token_count: 126
summary: The `kubeconfig` member of the ConfigMap is a config file with only the cluster information filled out. The key thing being communicated here is the `certificate-authority-data`. This may be expanded...
---

The `kubeconfig` member of the ConfigMap is a config file with only the cluster
information filled out. The key thing being communicated here is the
`certificate-authority-data`. This may be expanded in the future.
The signature is a JWS signature using the "detached" mode. To validate the
signature, the user should encode the `kubeconfig` payload according to JWS
rules (base64 encoded while discarding any trailing `=`). That encoded payload
is then used to form a whole JWS by inserting it between the 2 dots. You can
verify the JWS using the