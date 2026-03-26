---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Table of Contents
token_count: 255
summary: ## Table of Contents  - [Service Accounts](#service-accounts)   - [What are service accounts?](#what-are-service-accounts)     - [Default service accounts](#default-service-accounts)   - [Use cases...
---

## Table of Contents

- [Service Accounts](#service-accounts)
  - [What are service accounts?](#what-are-service-accounts)
    - [Default service accounts](#default-service-accounts)
  - [Use cases for Kubernetes service accounts](#use-cases-for-kubernetes-service-accounts)
    - [Grant permissions to a ServiceAccount](#grant-permissions-to-a-serviceaccount)
      - [Cross-namespace access using a ServiceAccount](#cross-namespace-access-using-a-serviceaccount)
    - [Assign a ServiceAccount to a Pod](#assign-a-serviceaccount-to-a-pod)
      - [Manually retrieve ServiceAccount credentials](#manually-retrieve-serviceaccount-credentials)
      - [Note:](#note)
    - [Restricting access to Secrets (deprecated)](#restricting-access-to-secrets-deprecated)
      - [Note:](#note)
  - [Authenticating service account credentials](#authenticating-service-account-credentials)
    - [Authenticating service account credentials in your own code](#authenticating-service-account-credentials-in-your-own-code)
  - [Alternatives](#alternatives)
  - [Feedback](#feedback)

---