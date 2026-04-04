---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#11-detailed
chunk_level: detailed
chunk_type: table
heading: Default roles and role bindings
token_count: 542
summary: ## Default roles and role bindings API servers create a set of default ClusterRole and ClusterRoleBinding objects. Many of these are `system:` prefixed, which indicates that the resource is directly...
---

## Default roles and role bindings
API servers create a set of default ClusterRole and ClusterRoleBinding objects.
Many of these are `system:` prefixed, which indicates that the resource is directly
managed by the cluster control plane.
All of the default ClusterRoles and ClusterRoleBindings are labeled with `kubernetes.io/bootstrapping=rbac-defaults`.
#### Caution:
Take care when modifying ClusterRoles and ClusterRoleBindings with names
that have a `system:` prefix.
Modifications to these resources can result in non-functional clusters.
### Auto-reconciliation
At each start-up, the API server updates default cluster roles with any missing permissions,
and updates default cluster role bindings with any missing subjects.
This allows the cluster to repair accidental modifications, and helps to keep roles and role bindings
up-to-date as permissions and subjects change in new Kubernetes releases.
To opt out of this reconciliation, set the `rbac.authorization.kubernetes.io/autoupdate`
annotation on a default cluster role or default cluster RoleBinding to `false`.
Be aware that missing default permissions and subjects can result in non-functional clusters.
Auto-reconciliation is enabled by default if the RBAC authorizer is active.
### API discovery roles
Default cluster role bindings authorize unauthenticated and authenticated users to read API information
that is deemed safe to be publicly accessible (including CustomResourceDefinitions).
To disable anonymous unauthenticated access, add `--anonymous-auth=false` flag to
the API server configuration.
To view the configuration of these roles via `kubectl` run:
```
`kubectl get clusterroles system:discovery -o yaml
`
```
#### Note:
If you edit that ClusterRole, your changes will be overwritten on API server restart
via [auto-reconciliation](#auto-reconciliation). To avoid that overwriting,
either do not manually edit the role, or disable auto-reconciliation.
Kubernetes RBAC API discovery roles|Default ClusterRole|Default ClusterRoleBinding|Description|
|**system:basic-user**|**system:authenticated** group|Allows a user read-only access to basic information about themselves. Prior to v1.14, this role was also bound to system:unauthenticated by default.|
|**system:discovery**|**system:authenticated** group|Allows read-only access to API discovery endpoints needed to discover and negotiate an API level. Prior to v1.14, this role was also bound to system:unauthenticated by default.|
|**system:public-info-viewer**|**system:authenticated** and **system:unauthenticated** groups|Allows read-only access to non-sensitive information about the cluster. Introduced in Kubernetes v1.14.|