---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#23-standard
chunk_level: standard
chunk_type: table
heading: Default roles and role bindings
token_count: 302
summary: ### API discovery roles Default cluster role bindings authorize unauthenticated and authenticated users to read API information that is deemed safe to be publicly accessible (including...
---

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