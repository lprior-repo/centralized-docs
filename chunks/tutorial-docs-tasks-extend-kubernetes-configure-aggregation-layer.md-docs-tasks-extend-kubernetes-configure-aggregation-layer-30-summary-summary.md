---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#30-summary
chunk_level: summary
chunk_type: prose
heading: Authentication Flow
token_count: 126
summary: The extension apiserver now can validate that the user/group retrieved from the headers are authorized to execute the given request. It does so by sending a standard...
---

The extension apiserver now can validate that the user/group retrieved from
the headers are authorized to execute the given request. It does so by sending
a standard [SubjectAccessReview](/docs/reference/access-authn-authz/authorization/)
request to the Kubernetes apiserver.
In order for the extension apiserver to be authorized itself to submit the
`SubjectAccessReview` request to the Kubernetes apiserver, it needs the correct permissions.
Kubernetes includes a default `ClusterRole` named `system:auth-delegator` that
has the appropriate permissions. It can be granted to the extension apiserver's service account.