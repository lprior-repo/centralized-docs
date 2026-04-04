---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#35-standard
chunk_level: standard
chunk_type: prose
heading: ServiceAccount permissions
token_count: 362
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
3. Grant a role to all service accounts in a namespace
If you want all applications in a namespace to have a role, no matter what service account they use,
you can grant a role to the service account group for that namespace.
For example, grant read-only permission within "my-namespace" to all service accounts in that namespace:
```
`kubectl create rolebinding serviceaccounts-view \\
--clusterrole=view \\
--group=system:serviceaccounts:my-namespace \\
--namespace=my-namespace
`
```
4. Grant a limited role to all service accounts cluster-wide (discouraged)
If you don't want to manage permissions per-namespace, you can grant a cluster-wide role to all service accounts.
For example, grant read-only permission across all namespaces to all service accounts in the cluster:
```
`kubectl create clusterrolebinding serviceaccounts-view \\
--clusterrole=view \\
--group=system:serviceaccounts
`
```
5. Grant super-user access to all service accounts cluster-wide (strongly discouraged)
If you don't care about partitioning permissions at all, you can grant super-user access to all service accounts.
#### Warning:
This allows any application full access to your cluster, and also grants
any user with read access to Secrets (or the ability to create any pod)
full access to your cluster.
```
`kubectl create clusterrolebinding serviceaccounts-cluster-admin \\
--clusterrole=cluster-admin \\
--group=system:serviceaccounts
`
```