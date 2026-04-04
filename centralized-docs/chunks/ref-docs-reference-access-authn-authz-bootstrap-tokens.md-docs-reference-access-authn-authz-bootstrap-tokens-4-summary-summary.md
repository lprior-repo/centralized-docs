---
doc_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens
chunk_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens#4-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 128
summary: FEATURE STATE: `Kubernetes v1.18 [stable]` Bootstrap tokens are a simple bearer token that is meant to be used when creating new clusters or joining new nodes to an existing cluster. It was built to...
---

FEATURE STATE:
`Kubernetes v1.18 [stable]`
Bootstrap tokens are a simple bearer token that is meant to be used when
creating new clusters or joining new nodes to an existing cluster.
It was built to support [kubeadm](/docs/reference/setup-tools/kubeadm/), but can be used in other contexts
for users that wish to start clusters without `kubeadm`. It is also built to
work, via RBAC policy, with the
[kubelet TLS Bootstrapping](/docs/reference/access-authn-authz/kubelet-tls-bootstrapping/) system.