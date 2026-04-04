---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#36-summary
chunk_level: summary
chunk_type: prose
heading: Bound service account token volume mechanism
token_count: 96
summary: ## Bound service account token volume mechanism FEATURE STATE: `Kubernetes v1.22 [stable]`(enabled by default) By default, the Kubernetes control plane (specifically, the [ServiceAccount admission...
---

## Bound service account token volume mechanism
FEATURE STATE:
`Kubernetes v1.22 [stable]`(enabled by default)
By default, the Kubernetes control plane (specifically, the
[ServiceAccount admission controller](#serviceaccount-admission-controller))
adds a [projected volume](/docs/concepts/storage/projected-volumes/) to Pods,
and this volume includes a token for Kubernetes API access.
Here's an example of how that looks for a launched Pod: