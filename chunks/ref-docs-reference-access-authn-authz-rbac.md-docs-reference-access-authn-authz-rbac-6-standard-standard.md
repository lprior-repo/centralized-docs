---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#6-standard
chunk_level: standard
chunk_type: prose
heading: API objects
token_count: 140
summary: ## API objects The RBAC API declares four kinds of Kubernetes object: *Role*, *ClusterRole*, *RoleBinding* and *ClusterRoleBinding*. You can describe or amend the RBAC...
---

## API objects
The RBAC API declares four kinds of Kubernetes object: *Role*, *ClusterRole*,
*RoleBinding* and *ClusterRoleBinding*. You can describe or amend the RBAC
[objects](/docs/concepts/overview/working-with-objects/#kubernetes-objects)
using tools such as `kubectl`, just like any other Kubernetes object.
#### Caution:
These objects, by design, impose access restrictions. If you are making changes
to a cluster as you learn, see
[privilege escalation prevention and bootstrapping](#privilege-escalation-prevention-and-bootstrapping)
to understand how those restrictions can prevent you making some changes.