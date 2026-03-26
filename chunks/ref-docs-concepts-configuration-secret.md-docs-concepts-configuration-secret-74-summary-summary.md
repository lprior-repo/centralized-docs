---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#74-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 102
summary: The `bootstrap.kubernetes.io/token` Secret type is for tokens used during the node bootstrap process. It stores tokens used to sign well-known ConfigMaps. A bootstrap token Secret is usually created...
---

The `bootstrap.kubernetes.io/token` Secret type is for
tokens used during the node bootstrap process. It stores tokens used to sign
well-known ConfigMaps.
A bootstrap token Secret is usually created in the `kube-system` namespace and
named in the form `bootstrap-token-&lt;token-id&gt;` where `&lt;token-id&gt;` is a 6 character
string of the token ID.
As a Kubernetes manifest, a bootstrap token Secret might look like the
following: