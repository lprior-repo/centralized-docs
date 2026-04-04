---
doc_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens
chunk_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens#5-summary
chunk_level: summary
chunk_type: prose
heading: Bootstrap Tokens Overview
token_count: 97
summary: ## Bootstrap Tokens Overview Bootstrap Tokens are defined with a specific type (`bootstrap.kubernetes.io/token`) of secrets that lives in the `kube-system` namespace. These Secrets are then read by...
---

## Bootstrap Tokens Overview
Bootstrap Tokens are defined with a specific type
(`bootstrap.kubernetes.io/token`) of secrets that lives in the `kube-system`
namespace. These Secrets are then read by the Bootstrap Authenticator in the
API Server. Expired tokens are removed with the TokenCleaner controller in the
Controller Manager. The tokens are also used to create a signature for a
specific ConfigMap used in a "discovery" process through a BootstrapSigner
controller.