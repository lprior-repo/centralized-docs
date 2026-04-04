---
doc_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster#21-summary
chunk_level: summary
chunk_type: prose
heading: Controlling access to the Kubernetes API
token_count: 112
summary: roles are provided that offer reasonable default separation of responsibility depending on what actions a client might want to perform. It is recommended that you use the...
---

roles are provided that offer reasonable default separation of responsibility depending on what
actions a client might want to perform. It is recommended that you use the
[Node](/docs/reference/access-authn-authz/node/) and
[RBAC](/docs/reference/access-authn-authz/rbac/) authorizers together, in combination with the
[NodeRestriction](/docs/reference/access-authn-authz/admission-controllers/#noderestriction) admission plugin.
As with authentication, simple and broad roles may be appropriate for smaller clusters, but as