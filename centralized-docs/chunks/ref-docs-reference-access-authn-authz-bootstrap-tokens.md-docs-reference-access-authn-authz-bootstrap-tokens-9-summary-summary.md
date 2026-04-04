---
doc_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens
chunk_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens#9-summary
chunk_level: summary
chunk_type: prose
heading: Bootstrap Token Secret Format
token_count: 63
summary: ## Bootstrap Token Secret Format Each valid token is backed by a secret in the `kube-system` namespace. You can find the full design doc...
---

## Bootstrap Token Secret Format
Each valid token is backed by a secret in the `kube-system` namespace. You can
find the full design doc
[here](https://git.k8s.io/design-proposals-archive/cluster-lifecycle/bootstrap-discovery.md).
Here is what the secret looks like.