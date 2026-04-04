---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#14-summary
chunk_level: summary
chunk_type: prose
heading: User accounts versus service accounts
token_count: 128
summary: * Auditing considerations for humans and service accounts may differ; the separation makes that easier to achieve. * A configuration bundle for a complex system may include definition of various...
---

* Auditing considerations for humans and service accounts may differ; the separation
makes that easier to achieve.
* A configuration bundle for a complex system may include definition of various service
accounts for components of that system. Because service accounts can be created
without many constraints and have namespaced names, such configuration is
usually portable.## Bound service account tokens
ServiceAccount tokens can be bound to API objects that exist in the kube-apiserver.
This can be used to tie the validity of a token to the existence of another API object.
Supported object types are as follows:
* Pod (used for projected volume mounts, see below)