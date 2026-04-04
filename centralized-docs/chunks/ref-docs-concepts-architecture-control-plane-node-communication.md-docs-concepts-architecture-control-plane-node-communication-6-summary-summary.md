---
doc_id: ref/docs-concepts-architecture-control-plane-node-communication.md/docs-concepts-architecture-control-plane-node-communication
chunk_id: ref/docs-concepts-architecture-control-plane-node-communication.md/docs-concepts-architecture-control-plane-node-communication#6-summary
chunk_level: summary
chunk_type: prose
heading: Node to Control Plane
token_count: 121
summary: enabled. One or more forms of [authorization](/docs/reference/access-authn-authz/authorization/) should be enabled, especially if [anonymous...
---

 enabled.
One or more forms of [authorization](/docs/reference/access-authn-authz/authorization/) should be
enabled, especially if [anonymous requests](/docs/reference/access-authn-authz/authentication/#anonymous-requests)
or [service account tokens](/docs/reference/access-authn-authz/authentication/#service-account-tokens)
are allowed.
Nodes should be provisioned with the public root [certificate](/docs/tasks/tls/managing-tls-in-a-cluster/) for the cluster such that they can
connect securely to the API server along with valid client credentials. A good approach is that the