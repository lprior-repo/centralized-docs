---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#16-detailed
chunk_level: detailed
chunk_type: code
heading: ServiceAccount permissions
token_count: 790
summary: ## ServiceAccount permissions Default RBAC policies grant scoped permissions to control-plane components, nodes, and controllers, but grant *no permissions* to service accounts outside the...
---

## ServiceAccount permissions
Default RBAC policies grant scoped permissions to control-plane components, nodes,
and controllers, but grant *no permissions* to service accounts outside the `kube-system` namespace
(beyond the permissions given by [API discovery roles](#discovery-roles)).
This allows you to grant particular roles to particular ServiceAccounts as needed.
Fine-grained role bindings provide greater security, but require more effort to administrate.
Broader grants can give unnecessary (and potentially escalating) API access to
ServiceAccounts, but are easier to administrate.
In order from most secure to least secure, the approaches are:
1. Grant a role to an application-specific service account (best practice)
This requires the application to specify a `serviceAccountName` in its pod spec,
and for the service account to be created (via the API, application manifest, `kubectl create serviceaccount`, etc.).
For example, grant read-only permission within "my-namespace" to the "my-sa" service account:
```
`kubectl create rolebinding my-sa-view \\
--clusterrole=view \\
--serviceaccount=my-namespace:my-sa \\
--namespace=my-namespace
`
```
2. Grant a role to the "default" service account in a namespace
If an application does not specify a `serviceAccountName`, it uses the "default" service account.
#### Note:
Permissions given to the "default" service account are available to any pod
in the namespace that does not specify a `serviceAccountName`.
For example, grant read-only permission within "my-namespace" to the "default" service account:
```
`kubectl create rolebinding default-view \\
--clusterrole=view \\
--serviceaccount=my-namespace:default \\
--namespace=my-namespace
`
```
Many [add-ons](/docs/concepts/cluster-administration/addons/) run as the
"default" service account in the `kube-system` namespace.
To allow those add-ons to run with super-user access, grant cluster-admin
permissions to the "default" service account in the `kube-system` namespace.
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