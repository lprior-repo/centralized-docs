---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#10-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 89
summary: * `Impersonate-User`: The username to act as. * `Impersonate-Uid`: A unique identifier that represents the user being impersonated. Optional. Requires \"Impersonate-User\". Kubernetes does not impose...
---

* `Impersonate-User`: The username to act as.
* `Impersonate-Uid`: A unique identifier that represents the user being impersonated. Optional.
Requires "Impersonate-User". Kubernetes does not impose any format requirements on this string.
* `Impersonate-Group`: A group name to act as. Can be provided multiple times to set multiple groups.
Optional. Requires "Impersonate-User".