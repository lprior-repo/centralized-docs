---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#46-summary
chunk_level: summary
chunk_type: table
heading: Types of volumes
token_count: 107
summary: You can restrict the use of `gitRepo` volumes in your cluster using [policies](/docs/concepts/policy/), such as...
---

You can restrict the use of `gitRepo` volumes in your cluster using
[policies](/docs/concepts/policy/), such as
[ValidatingAdmissionPolicy](/docs/reference/access-authn-authz/validating-admission-policy/).
You can use the following Common Expression Language (CEL) expression as
part of a policy to reject use of `gitRepo` volumes:
```
`!has(object.spec.volumes) || !object.spec.volumes.exists(v, has(v.gitRepo))
`
```