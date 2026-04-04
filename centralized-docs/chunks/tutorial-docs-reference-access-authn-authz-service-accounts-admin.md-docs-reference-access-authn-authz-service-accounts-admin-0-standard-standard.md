---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#0-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 442
summary: ## Table of Contents  - [Managing Service Accounts](#managing-service-accounts)   - [Before you begin](#before-you-begin)   - [User accounts versus service...
---

## Table of Contents

- [Managing Service Accounts](#managing-service-accounts)
  - [Before you begin](#before-you-begin)
  - [User accounts versus service accounts](#user-accounts-versus-service-accounts)
    - [Additional metadata in Pod bound tokens](#additional-metadata-in-pod-bound-tokens)
    - [Verifying and inspecting private claims](#verifying-and-inspecting-private-claims)
      - [Note:](#note)
      - [Schema for service account private claims](#schema-for-service-account-private-claims)
      - [Note:](#note)
  - [Bound service account token volume mechanism](#bound-service-account-token-volume-mechanism)
      - [Note:](#note)
  - [Manual Secret management for ServiceAccounts](#manual-secret-management-for-serviceaccounts)
      - [Note:](#note)
  - [Auto-generated legacy ServiceAccount token clean up](#auto-generated-legacy-serviceaccount-token-clean-up)
    - [ServiceAccount controller](#serviceaccount-controller)
    - [Token controller](#token-controller)
    - [ServiceAccount admission controller](#serviceaccount-admission-controller)
    - [Legacy ServiceAccount token cleaner](#legacy-serviceaccount-token-cleaner)
      - [Note:](#note)
    - [TokenRequest API](#tokenrequest-api)
      - [Caution:](#caution)
      - [Caution:](#caution)
    - [Delete/invalidate a long-lived/legacy ServiceAccount token](#deleteinvalidate-a-long-livedlegacy-serviceaccount-token)
    - [Delete/invalidate a short-lived ServiceAccount token](#deleteinvalidate-a-short-lived-serviceaccount-token)
  - [External ServiceAccount token signing and key management](#external-serviceaccount-token-signing-and-key-management)
      - [Note:](#note)
    - [Metadata](#metadata)
    - [FetchKeys](#fetchkeys)
    - [Sign](#sign)
  - [Clean up](#clean-up)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---