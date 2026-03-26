---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#0-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 507
summary: ## Table of Contents  - [User Impersonation](#user-impersonation)       - [Note:](#note)       - [Note:](#note) - [Can impersonate the user...
---

## Table of Contents

- [User Impersonation](#user-impersonation)
      - [Note:](#note)
      - [Note:](#note)
- [Can impersonate the user "jane.doe@example.com"](#can-impersonate-the-user-janedoeexamplecom)
- [Can impersonate the groups "developers" and "admins"](#can-impersonate-the-groups-developers-and-admins)
- [Can impersonate the extras field "scopes" with the values "view" and "development"](#can-impersonate-the-extras-field-scopes-with-the-values-view-and-development)
- [Can impersonate the uid "06f6ce97-e2c5-4ab8-7ba5-7654dd08d52b"](#can-impersonate-the-uid-06f6ce97-e2c5-4ab8-7ba5-7654dd08d52b)
      - [Note:](#note)
  - [Constrained Impersonation](#constrained-impersonation)
    - [Understanding constrained impersonation](#understanding-constrained-impersonation)
    - [Impersonation modes](#impersonation-modes)
      - [user-info mode](#user-info-mode)
      - [Note:](#note)
    - [Configuring constrained impersonation with RBAC](#configuring-constrained-impersonation-with-rbac)
      - [Example: Impersonate a user for specific actions](#example-impersonate-a-user-for-specific-actions)
      - [Example: Impersonate a ServiceAccount](#example-impersonate-a-serviceaccount)
- [For service accounts, you must specify the namespace in the RoleBinding](#for-service-accounts-you-must-specify-the-namespace-in-the-rolebinding)
      - [Example: Impersonate a node](#example-impersonate-a-node)
      - [Example: Node agent impersonating the associated node](#example-node-agent-impersonating-the-associated-node)
    - [Using constrained impersonation](#using-constrained-impersonation)
    - [Working with `impersonate` verb](#working-with-impersonate-verb)
  - [Auditing](#auditing)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---