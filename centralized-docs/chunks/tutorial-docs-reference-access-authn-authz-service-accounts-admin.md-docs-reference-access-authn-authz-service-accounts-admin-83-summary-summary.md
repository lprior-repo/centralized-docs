---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#83-summary
chunk_level: summary
chunk_type: prose
heading: External ServiceAccount token signing and key management
token_count: 126
summary: FEATURE STATE: `Kubernetes v1.34 [beta]`(enabled by default) The kube-apiserver can be configured to use external signer for token signing and token verifying key management. This feature enables...
---

FEATURE STATE:
`Kubernetes v1.34 [beta]`(enabled by default)
The kube-apiserver can be configured to use external signer for token signing and token verifying key management.
This feature enables kubernetes distributions to integrate with key management solutions of their choice
(for example, HSMs, cloud KMSes) for service account credential signing and verification.
To configure kube-apiserver to use external-jwt-signer set the `--service-account-signing-endpoint` flag
to the location of a Unix domain socket (UDS) on a filesystem, or be prefixed with an @ symbol and name