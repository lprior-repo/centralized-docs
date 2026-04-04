---
doc_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens
chunk_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Table of Contents
token_count: 235
summary: ## Table of Contents  - [Authenticating with Bootstrap Tokens](#authenticating-with-bootstrap-tokens)   - [Bootstrap Tokens Overview](#bootstrap-tokens-overview)   - [Token Format](#token-format)   -...
---

## Table of Contents

- [Authenticating with Bootstrap Tokens](#authenticating-with-bootstrap-tokens)
  - [Bootstrap Tokens Overview](#bootstrap-tokens-overview)
  - [Token Format](#token-format)
  - [Enabling Bootstrap Token Authentication](#enabling-bootstrap-token-authentication)
  - [Bootstrap Token Secret Format](#bootstrap-token-secret-format)
- [Name MUST be of form "bootstrap-token-&lt;token id&gt;"](#name-must-be-of-form-bootstrap-token-lttoken-idgt)
- [Human readable description. Optional.](#human-readable-description-optional)
- [Token ID and secret. Required.](#token-id-and-secret-required)
- [Extra groups to authenticate the token as. Must start with "system:bootstrappers:"](#extra-groups-to-authenticate-the-token-as-must-start-with-systembootstrappers)
  - [Token Management with kubeadm](#token-management-with-kubeadm)
  - [ConfigMap Signing](#configmap-signing)
      - [Warning:](#warning)
  - [Feedback](#feedback)

---