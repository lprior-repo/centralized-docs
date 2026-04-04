---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#0-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 378
summary: ## Table of Contents  - [Certificates and Certificate Signing Requests](#certificates-and-certificate-signing-requests)   - [Certificate signing requests](#certificate-signing-requests)     -...
---

## Table of Contents

- [Certificates and Certificate Signing Requests](#certificates-and-certificate-signing-requests)
  - [Certificate signing requests](#certificate-signing-requests)
    - [Request signing process](#request-signing-process)
  - [Signers](#signers)
      - [Note:](#note)
    - [Kubernetes signers](#kubernetes-signers)
      - [Note:](#note)
    - [Custom signers](#custom-signers)
    - [Control plane signer](#control-plane-signer)
      - [Note:](#note)
      - [Note:](#note)
    - [API-based signers](#api-based-signers)
  - [Approval or rejection](#approval-or-rejection)
    - [Control plane automated approval](#control-plane-automated-approval)
    - [Approval or rejection using `kubectl`](#approval-or-rejection-using-kubectl)
    - [Approval or rejection using the Kubernetes API](#approval-or-rejection-using-the-kubernetes-api)
      - [Note:](#note)
  - [Cluster trust bundles](#cluster-trust-bundles)
      - [Note:](#note)
    - [Common properties and validation](#common-properties-and-validation)
    - [Signer-linked ClusterTrustBundles](#signer-linked-clustertrustbundles)
    - [Signer-unlinked ClusterTrustBundles](#signer-unlinked-clustertrustbundles)
- [no signerName specified, so the field is blank](#no-signername-specified-so-the-field-is-blank)
    - [Accessing ClusterTrustBundles from pods](#accessing-clustertrustbundles-from-pods)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---