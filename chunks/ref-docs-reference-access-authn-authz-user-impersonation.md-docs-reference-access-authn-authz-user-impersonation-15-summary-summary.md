---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#15-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 115
summary: When using `kubectl` set the `--as` command line argument to configure the `Impersonate-User` header, you can also set the `--as-group` flag to configure the `Impersonate-Group` header， set the...
---

When using `kubectl` set the `--as` command line argument to configure the `Impersonate-User`
header, you can also set the `--as-group` flag to configure the `Impersonate-Group` header，
set the `--as-uid` flag (1.23) to configure `Impersonate-Uid` header, and set the
`--as-user-extra` flag (1.35) to configure `Impersonate-Extra-( extra name )` header.
```
`kubectl drain mynode
`
```