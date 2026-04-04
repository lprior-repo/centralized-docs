---
doc_id: ref/docs-reference-kubernetes-api.md/docs-reference-kubernetes-api
chunk_id: ref/docs-reference-kubernetes-api.md/docs-reference-kubernetes-api#0-standard
chunk_level: standard
chunk_type: prose
heading: Related Pages
token_count: 410
summary: ## Table of Contents  - [Kubernetes API](#kubernetes-api)   - [Feedback](#feedback)  ---  # Kubernetes API Kubernetes' API is the application that serves Kubernetes functionality through a RESTful...
---

## Table of Contents

- [Kubernetes API](#kubernetes-api)
  - [Feedback](#feedback)

---

# Kubernetes API
Kubernetes' API is the application that serves Kubernetes functionality through a RESTful interface and stores the state of the cluster.
Kubernetes resources and "records of intent" are all stored as API objects, and modified via RESTful calls to the API. The API allows configuration to be managed in a declarative way. Users can interact with the Kubernetes API directly, or via tools like `kubectl`. The core Kubernetes API is flexible and can also be extended to support custom resources.
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified July 12, 2023 at 1:25 AM PST: [Revise docs home page (9520b96a61)](https://github.com/kubernetes/website/commit/9520b96a6162d4af841da63227d2a8710596b975)
## Related Pages

- [Binding](docs-reference-kubernetes-api-workload-resources-binding-v1.md)
- [conventions](docs-reference-kubectl-conventions.md)
- [HorizontalPodAutoscaler](docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)
- [Концепции](ru-docs-concepts.md)
- [Using RBAC Authorization](docs-reference-access-authn-authz-rbac.md)