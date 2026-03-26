---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#143-summary
chunk_level: summary
chunk_type: prose
heading: ServiceAccount permissions
token_count: 64
summary: Many [add-ons](/docs/concepts/cluster-administration/addons/) run as the \"default\" service account in the `kube-system` namespace. To allow those add-ons to run with super-user access, grant...
---

Many [add-ons](/docs/concepts/cluster-administration/addons/) run as the
"default" service account in the `kube-system` namespace.
To allow those add-ons to run with super-user access, grant cluster-admin
permissions to the "default" service account in the `kube-system` namespace.