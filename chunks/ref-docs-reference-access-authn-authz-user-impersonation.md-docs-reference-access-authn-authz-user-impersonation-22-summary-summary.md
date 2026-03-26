---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#22-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 126
summary: - apiGroups: [\"\"] resources: [\"groups\"] verbs: [\"impersonate\"] resourceNames: [\"developers\",\"admins\"] # Can impersonate the extras field \"scopes\" with the values \"view\" and \"development\" - apiGroups:...
---

- apiGroups: [""]
resources: ["groups"]
verbs: ["impersonate"]
resourceNames: ["developers","admins"]
# Can impersonate the extras field "scopes" with the values "view" and "development"
- apiGroups: ["authentication.k8s.io"]
resources: ["userextras/scopes"]
verbs: ["impersonate"]
resourceNames: ["view", "development"]
# Can impersonate the uid "06f6ce97-e2c5-4ab8-7ba5-7654dd08d52b"
- apiGroups: ["authentication.k8s.io"]