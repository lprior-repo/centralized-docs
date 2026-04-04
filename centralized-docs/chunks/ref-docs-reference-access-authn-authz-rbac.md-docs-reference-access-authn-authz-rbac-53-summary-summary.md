---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#53-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 109
summary: ``` `GET /api/v1/namespaces/{namespace}/pods/{name}/log ` ``` In this case, `pods` is the namespaced resource for Pod resources, and `log` is a subresource of `pods`. To represent this in an RBAC...
---

```
`GET /api/v1/namespaces/{namespace}/pods/{name}/log
`
```
In this case, `pods` is the namespaced resource for Pod resources, and `log` is a
subresource of `pods`. To represent this in an RBAC role, use a slash (`/`) to
delimit the resource and subresource. To allow a subject to read `pods` and
also access the `log` subresource for each of those Pods, you write: