---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#52-summary
chunk_level: summary
chunk_type: prose
heading: Alternatives
token_count: 120
summary: * [Use a service mesh such as Istio to provide certificates to Pods](https://istio.io/latest/docs/tasks/security/cert-management/plugin-ca-cert/). * Authenticate from outside the cluster to the API...
---

* [Use a service mesh such as Istio to provide certificates to Pods](https://istio.io/latest/docs/tasks/security/cert-management/plugin-ca-cert/).
* Authenticate from outside the cluster to the API server without using service account tokens:
* [Configure the API server to accept OpenID Connect (OIDC) tokens from your identity provider](/docs/reference/access-authn-authz/authentication/#openid-connect-tokens).
* Use service accounts or user accounts created using an external Identity
and Access Management (IAM) service, such as from a cloud provider, to
authenticate to your cluster.