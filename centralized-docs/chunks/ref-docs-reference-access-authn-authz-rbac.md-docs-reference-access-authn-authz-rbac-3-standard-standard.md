---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#3-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 386
summary: - [objects is \"deployments\"](#objects-is-deployments) - [at the HTTP level, the name of the resource for accessing Pod](#at-the-http-level-the-name-of-the-resource-for-accessing-pod) - [objects is...
---

- [objects is "deployments"](#objects-is-deployments)
- [at the HTTP level, the name of the resource for accessing Pod](#at-the-http-level-the-name-of-the-resource-for-accessing-pod)
- [objects is "pods"](#objects-is-pods)
- [at the HTTP level, the name of the resource for accessing Job](#at-the-http-level-the-name-of-the-resource-for-accessing-job)
- [objects is "jobs"](#objects-is-jobs)
- [at the HTTP level, the name of the resource for accessing ConfigMap](#at-the-http-level-the-name-of-the-resource-for-accessing-configmap)
- [objects is "configmaps"](#objects-is-configmaps)
- [at the HTTP level, the name of the resource for accessing Node](#at-the-http-level-the-name-of-the-resource-for-accessing-node)
- [objects is "nodes"](#objects-is-nodes)
    - [Referring to subjects](#referring-to-subjects)
      - [Caution:](#caution)
      - [Note:](#note)
      - [RoleBinding examples](#rolebinding-examples)
  - [Default roles and role bindings](#default-roles-and-role-bindings)
      - [Caution:](#caution)
    - [Auto-reconciliation](#auto-reconciliation)
    - [API discovery roles](#api-discovery-roles)
      - [Note:](#note)
    - [User-facing roles](#user-facing-roles)
    - [Core component roles](#core-component-roles)
    - [Other component roles](#other-component-roles)
    - [Roles for built-in controllers](#roles-for-built-in-controllers)
    - [Restrictions on role creation or update](#restrictions-on-role-creation-or-update)