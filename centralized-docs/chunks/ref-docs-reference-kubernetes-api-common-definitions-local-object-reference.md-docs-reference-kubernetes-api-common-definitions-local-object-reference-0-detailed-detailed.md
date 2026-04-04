---
doc_id: ref/docs-reference-kubernetes-api-common-definitions-local-object-reference.md/docs-reference-kubernetes-api-common-definitions-local-object-reference
chunk_id: ref/docs-reference-kubernetes-api-common-definitions-local-object-reference.md/docs-reference-kubernetes-api-common-definitions-local-object-reference#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 494
summary: ## Table of Contents  - [LocalObjectReference](#localobjectreference)   - [Feedback](#feedback)  ---  # LocalObjectReference LocalObjectReference contains enough information to let you locate the...
---

## Table of Contents

- [LocalObjectReference](#localobjectreference)
  - [Feedback](#feedback)

---

# LocalObjectReference
LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
`import "k8s.io/api/core/v1"`
LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
* **name** (string)
Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: [https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names](https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names)
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
Last modified August 24, 2024 at 10:29 PM PST: [Update generated API reference for v1.31 (890b36a496)](https://github.com/kubernetes/website/commit/890b36a496fb93c68efedc06385293ee35326df7)
This page is automatically generated.
If you plan to report an issue with this page, mention that the page is auto-generated in your issue description. The fix may need to happen elsewhere in the Kubernetes project.
## Related Pages

- [HorizontalPodAutoscaler](docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)
- [Binding](docs-reference-kubernetes-api-workload-resources-binding-v1.md)
- [conventions](docs-reference-kubectl-conventions.md)
- [Концепции](ru-docs-concepts.md)
- [Using RBAC Authorization](docs-reference-access-authn-authz-rbac.md)