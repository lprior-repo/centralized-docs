---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#110-summary
chunk_level: summary
chunk_type: prose
heading: Default roles and role bindings
token_count: 125
summary: The Kubernetes [controller manager](/docs/reference/command-line-tools-reference/kube-controller-manager/) runs [controllers](/docs/concepts/architecture/controller/) that are built in to the...
---

The Kubernetes [controller manager](/docs/reference/command-line-tools-reference/kube-controller-manager/) runs
[controllers](/docs/concepts/architecture/controller/) that are built in to the Kubernetes
control plane.
When invoked with `--use-service-account-credentials`, kube-controller-manager starts each controller
using a separate service account.
Corresponding roles exist for each built-in controller, prefixed with `system:controller:`.
If the controller manager is not started with `--use-service-account-credentials`, it runs all control loops
using its own credential, which must be granted all the relevant roles.
These roles include: