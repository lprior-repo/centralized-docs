---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#84-summary
chunk_level: summary
chunk_type: prose
heading: External ServiceAccount token signing and key management
token_count: 88
summary: flag to the location of a Unix domain socket (UDS) on a filesystem, or be prefixed with an @ symbol and name a UDS in the abstract socket namespace. At the configured UDS shall be an RPC server which...
---

 flag
to the location of a Unix domain socket (UDS) on a filesystem, or be prefixed with an @ symbol and name
a UDS in the abstract socket namespace. At the configured UDS shall be an RPC server which implements
an `ExternalJWTSigner` gRPC service.
The external-jwt-signer must be healthy and be ready to serve supported service account keys for the kube-apiserver to start.