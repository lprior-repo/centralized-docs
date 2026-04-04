---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#57-summary
chunk_level: summary
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 108
summary: * the admission controller mutates the incoming Pod, adding an extra [volume](/docs/concepts/storage/volumes/) that contains a token for API access. * the admission controller adds a `volumeMount` to...
---

* the admission controller mutates the incoming Pod, adding an extra
[volume](/docs/concepts/storage/volumes/) that contains
a token for API access.
* the admission controller adds a `volumeMount` to each container in the Pod,
skipping any containers that already have a volume mount defined for the path
`/var/run/secrets/kubernetes.io/serviceaccount`.
For Linux containers, that volume is mounted at `/var/run/secrets/kubernetes.io/serviceaccount`;
on Windows nodes, the mount is at the equivalent path.