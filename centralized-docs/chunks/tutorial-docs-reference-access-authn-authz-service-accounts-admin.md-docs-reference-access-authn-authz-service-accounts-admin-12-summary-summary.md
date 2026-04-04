---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#12-summary
chunk_level: summary
chunk_type: prose
heading: User accounts versus service accounts
token_count: 93
summary: * User accounts are for humans. Service accounts are for application processes, which (for Kubernetes) run in containers that are part of pods. * User accounts are intended to be global: names must...
---

* User accounts are for humans. Service accounts are for application processes,
which (for Kubernetes) run in containers that are part of pods.
* User accounts are intended to be global: names must be unique across all
namespaces of a cluster. No matter what namespace you look at, a particular
username that represents a user represents the same user.
In Kubernetes, service accounts are namespaced: two different namespaces can
contain ServiceAccounts that have identical names.