---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#18-standard
chunk_level: standard
chunk_type: prose
heading: External ServiceAccount token signing and key management
token_count: 282
summary: ## External ServiceAccount token signing and key management FEATURE STATE: `Kubernetes v1.34 [beta]`(enabled by default) The kube-apiserver can be configured to use external signer for token signing...
---

## External ServiceAccount token signing and key management
FEATURE STATE:
`Kubernetes v1.34 [beta]`(enabled by default)
The kube-apiserver can be configured to use external signer for token signing and token verifying key management.
This feature enables kubernetes distributions to integrate with key management solutions of their choice
(for example, HSMs, cloud KMSes) for service account credential signing and verification.
To configure kube-apiserver to use external-jwt-signer set the `--service-account-signing-endpoint` flag
to the location of a Unix domain socket (UDS) on a filesystem, or be prefixed with an @ symbol and name
a UDS in the abstract socket namespace. At the configured UDS shall be an RPC server which implements
an `ExternalJWTSigner` gRPC service.
The external-jwt-signer must be healthy and be ready to serve supported service account keys for the kube-apiserver to start.
#### Note:
The kube-apiserver flags `--service-account-key-file` and `--service-account-signing-key-file` will continue
to be used for reading from files unless `--service-account-signing-endpoint` is set; they are mutually
exclusive ways of supporting JWT signing and authentication.
An external signer provides a `v1.ExternalJWTSigner` gRPC service that implements 3 methods: