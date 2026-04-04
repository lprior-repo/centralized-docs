---
doc_id: ref/docs-concepts-overview-working-with-objects-field-selectors.md/docs-concepts-overview-working-with-objects-field-selectors
chunk_id: ref/docs-concepts-overview-working-with-objects-field-selectors.md/docs-concepts-overview-working-with-objects-field-selectors#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 452
summary: ## Chained selectors As with [label](/docs/concepts/overview/working-with-objects/labels/) and other selectors, field selectors can be chained together as a comma-separated list. This `kubectl`...
---

## Chained selectors
As with [label](/docs/concepts/overview/working-with-objects/labels/) and other selectors, field selectors can be chained together as a comma-separated list. This `kubectl` command selects all Pods for which the `status.phase` does not equal `Running` and the `spec.restartPolicy` field equals `Always`:
```
`kubectl get pods --field-selector=status.phase!=Running,spec.restartPolicy=Always
`
```
## Multiple resource types
You can use field selectors across multiple resource types. This `kubectl` command selects all Statefulsets and Services that are not in the `default` namespace:
```
`kubectl get statefulsets,services --all-namespaces --field-selector metadata.namespace!=default
`
```
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
Last modified November 06, 2025 at 12:00 AM PST: [Add missing status.podIPs (62da17b94e)](https://github.com/kubernetes/website/commit/62da17b94e11cf0b082ca55425b214980159b9b5)
## Related Pages

- [Using RBAC Authorization](docs-reference-access-authn-authz-rbac.md)
- [Annotations](docs-concepts-overview-working-with-objects-annotations.md)
- [Objects In Kubernetes](docs-concepts-overview-working-with-objects.md)
- [Validating Admission Policy](docs-reference-access-authn-authz-validating-admission-policy.md)
- [Automatic Cleanup for Finished Jobs](docs-concepts-workloads-controllers-ttlafterfinished.md)