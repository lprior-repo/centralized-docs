---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#13-summary
chunk_level: summary
chunk_type: prose
heading: User accounts versus service accounts
token_count: 115
summary: * Typically, a cluster's user accounts might be synchronised from a corporate database, where new user account creation requires special privileges and is tied to complex business processes. By...
---

* Typically, a cluster's user accounts might be synchronised from a corporate
database, where new user account creation requires special privileges and is
tied to complex business processes. By contrast, service account creation is
intended to be more lightweight, allowing cluster users to create service accounts
for specific tasks on demand. Separating ServiceAccount creation from the steps to
onboard human users makes it easier for workloads to follow the principle of
least privilege.
* Auditing considerations for humans and service accounts may differ; the separation
makes that easier to achieve.