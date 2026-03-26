---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#51-summary
chunk_level: summary
chunk_type: prose
heading: Alternatives
token_count: 124
summary: * Issue your own tokens using another mechanism, and then use [Webhook Token Authentication](/docs/reference/access-authn-authz/authentication/#webhook-token-authentication) to validate bearer tokens...
---

* Issue your own tokens using another mechanism, and then use
[Webhook Token Authentication](/docs/reference/access-authn-authz/authentication/#webhook-token-authentication)
to validate bearer tokens using your own validation service.
* Provide your own identities to Pods.
* [Use the SPIFFE CSI driver plugin to provide SPIFFE SVIDs as X.509 certificate pairs to Pods](https://cert-manager.io/docs/projects/csi-driver-spiffe/).
🛇 This item links to a third party project or product that is not part of Kubernetes itself. [More information](#third-party-content-disclaimer)