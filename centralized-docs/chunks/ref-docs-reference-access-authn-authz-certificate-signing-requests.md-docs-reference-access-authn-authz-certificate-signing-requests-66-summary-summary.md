---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#66-summary
chunk_level: summary
chunk_type: prose
heading: Approval or rejection
token_count: 118
summary: ## Approval or rejection Before a [signer](#signers) issues a certificate based on a CertificateSigningRequest, the signer typically checks that the issuance for that CSR has been *approved*. ###...
---

## Approval or rejection
Before a [signer](#signers) issues a certificate based on a CertificateSigningRequest,
the signer typically checks that the issuance for that CSR has been *approved*.
### Control plane automated approval
The kube-controller-manager ships with a built-in approver for certificates with
a signerName of `kubernetes.io/kube-apiserver-client-kubelet` that delegates various
permissions on CSRs for node credentials to authorization.
The kube-controller-manager POSTs SubjectAccessReview resources to the API server
in order to check authorization for certificate approval.