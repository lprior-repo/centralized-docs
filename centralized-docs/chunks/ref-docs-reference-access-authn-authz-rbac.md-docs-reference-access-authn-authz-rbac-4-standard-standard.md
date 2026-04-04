---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#4-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 219
summary: - [omit resourceNames to allow binding any ClusterRole](#omit-resourcenames-to-allow-binding-any-clusterrole)     - [`kubectl create role`](#kubectl-create-role)     - [`kubectl create...
---

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