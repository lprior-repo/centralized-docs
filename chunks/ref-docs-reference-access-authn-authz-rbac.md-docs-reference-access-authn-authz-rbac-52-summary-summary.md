---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#52-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 110
summary: ### Referring to resources In the Kubernetes API, most resources are represented and accessed using a string representation of their object name, such as `pods` for a Pod. RBAC refers to resources...
---

### Referring to resources
In the Kubernetes API, most resources are represented and accessed using a string representation of
their object name, such as `pods` for a Pod. RBAC refers to resources using exactly the same
name that appears in the URL for the relevant API endpoint.
Some Kubernetes APIs involve a
*subresource*, such as the logs for a Pod. A request for a Pod's logs looks like:
```
`GET /api/v1/namespaces/{namespace}/pods/{name}/log
`
```