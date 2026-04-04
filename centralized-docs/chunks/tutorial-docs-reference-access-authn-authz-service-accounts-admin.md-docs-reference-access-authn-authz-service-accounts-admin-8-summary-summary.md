---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#8-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 123
summary: # Managing Service Accounts A *ServiceAccount* provides an identity for processes that run in a Pod. A process inside a Pod can use the identity of its associated service account to authenticate to...
---

# Managing Service Accounts
A *ServiceAccount* provides an identity for processes that run in a Pod.
A process inside a Pod can use the identity of its associated service account to
authenticate to the cluster's API server.
For an introduction to service accounts, read [configure service accounts](/docs/tasks/configure-pod-container/configure-service-account/).
This task guide explains some of the concepts behind ServiceAccounts. The
guide also explains how to obtain or revoke tokens that represent
ServiceAccounts, and how to (optionally) bind a ServiceAccount's validity to
the lifetime of an API object.