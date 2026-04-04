---
doc_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler
chunk_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler#5-standard
chunk_level: standard
chunk_type: prose
heading: Related Pages
token_count: 326
summary: ## Disallow labeling nodes A cluster administrator should ensure that cluster users cannot label the nodes. A malicious actor can use `nodeSelector` to schedule workloads on nodes where those...
---

## Disallow labeling nodes
A cluster administrator should ensure that cluster users cannot label the nodes.
A malicious actor can use `nodeSelector` to schedule workloads on nodes where those workloads should not be present.
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
Last modified February 19, 2026 at 3:34 PM PST: [Fix some links in the En docs (95b7685f71)](https://github.com/kubernetes/website/commit/95b7685f7156c317aa59d86618e8ec4535d2015f)
## Related Pages

- [Kubernetes Scheduler](docs-concepts-scheduling-eviction-kube-scheduler.md)
- [Using RBAC Authorization](docs-reference-access-authn-authz-rbac.md)
- [Objects In Kubernetes](docs-concepts-overview-working-with-objects.md)
- [Binding](docs-reference-kubernetes-api-workload-resources-binding-v1.md)
- [conventions](docs-reference-kubectl-conventions.md)