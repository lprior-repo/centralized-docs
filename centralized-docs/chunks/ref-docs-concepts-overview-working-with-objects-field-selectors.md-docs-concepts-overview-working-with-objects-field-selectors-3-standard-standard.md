---
doc_id: ref/docs-concepts-overview-working-with-objects-field-selectors.md/docs-concepts-overview-working-with-objects-field-selectors
chunk_id: ref/docs-concepts-overview-working-with-objects-field-selectors.md/docs-concepts-overview-working-with-objects-field-selectors#3-standard
chunk_level: standard
chunk_type: prose
heading: Feedback
token_count: 473
summary: ## Supported operators You can use the `=`, `==`, and `!=` operators with field selectors (`=` and `==` mean the same thing). This `kubectl` command, for example, selects all Kubernetes Services that...
---

## Supported operators
You can use the `=`, `==`, and `!=` operators with field selectors (`=` and `==` mean the same thing). This `kubectl` command, for example, selects all Kubernetes Services that aren't in the `default` namespace:
```
`kubectl get services --all-namespaces --field-selector metadata.namespace!=default
`
```
#### Note:
[Set-based operators](/docs/concepts/overview/working-with-objects/labels/#set-based-requirement)
(`in`, `notin`, `exists`) are not supported for field selectors.
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