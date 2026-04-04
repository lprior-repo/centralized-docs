---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#91-summary
chunk_level: summary
chunk_type: table
heading: Default roles and role bindings
token_count: 97
summary: |**system:discovery**|**system:authenticated** group|Allows read-only access to API discovery endpoints needed to discover and negotiate an API level. Prior to v1.14, this role was also bound to...
---

|**system:discovery**|**system:authenticated** group|Allows read-only access to API discovery endpoints needed to discover and negotiate an API level. Prior to v1.14, this role was also bound to system:unauthenticated by default.|
|**system:public-info-viewer**|**system:authenticated** and **system:unauthenticated** groups|Allows read-only access to non-sensitive information about the cluster. Introduced in Kubernetes v1.14.|