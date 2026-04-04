---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#144-summary
chunk_level: summary
chunk_type: prose
heading: ServiceAccount permissions
token_count: 59
summary: #### Caution: Enabling this means the `kube-system` namespace contains Secrets that grant super-user access to your cluster's API. ``` `kubectl create clusterrolebinding add-on-cluster-admin \\...
---

#### Caution:
Enabling this means the `kube-system` namespace contains Secrets
that grant super-user access to your cluster's API.
```
`kubectl create clusterrolebinding add-on-cluster-admin \\
--clusterrole=cluster-admin \\
--serviceaccount=kube-system:default
`
```