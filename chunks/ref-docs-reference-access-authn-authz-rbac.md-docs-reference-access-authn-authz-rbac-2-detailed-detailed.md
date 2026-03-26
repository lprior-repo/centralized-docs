---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Table of Contents
token_count: 605
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
- [omit resourceNames to allow binding any ClusterRole](#omit-resourcenames-to-allow-binding-any-clusterrole)
    - [`kubectl create role`](#kubectl-create-role)
    - [`kubectl create clusterrole`](#kubectl-create-clusterrole)
    - [`kubectl create rolebinding`](#kubectl-create-rolebinding)
    - [`kubectl create clusterrolebinding`](#kubectl-create-clusterrolebinding)
    - [`kubectl auth reconcile`](#kubectl-auth-reconcile)
  - [ServiceAccount permissions](#serviceaccount-permissions)
      - [Note:](#note)
      - [Caution:](#caution)
      - [Warning:](#warning)
  - [Write access for EndpointSlices](#write-access-for-endpointslices)
  - [Upgrading from ABAC](#upgrading-from-abac)
    - [Parallel authorizers](#parallel-authorizers)
    - [Permissive RBAC permissions](#permissive-rbac-permissions)
      - [Warning:](#warning)
  - [Feedback](#feedback)

---