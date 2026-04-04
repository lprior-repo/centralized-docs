---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#24-standard
chunk_level: standard
chunk_type: prose
heading: Related Pages
token_count: 512
summary: ## API kinds exempt from admission validation There are certain API kinds that are exempt from admission-time validation checks. For example, you can't create a ValidatingAdmissionPolicy that...
---

## API kinds exempt from admission validation
There are certain API kinds that are exempt from admission-time validation checks. For example, you can't create a ValidatingAdmissionPolicy that prevents changes to ValidatingAdmissionPolicyBindings.
The list of exempt API kinds is:
* [ValidatingAdmissionPolicies](/docs/reference/kubernetes-api/policy-resources/validating-admission-policy-v1/)
* [ValidatingAdmissionPolicyBindings](/docs/reference/kubernetes-api/policy-resources/validating-admission-policy-binding-v1/)
* MutatingAdmissionPolicies
* MutatingAdmissionPolicyBindings
* [TokenReviews](/docs/reference/kubernetes-api/authentication-resources/token-review-v1/)
* [LocalSubjectAccessReviews](/docs/reference/kubernetes-api/authorization-resources/local-subject-access-review-v1/)
* [SelfSubjectAccessReviews](/docs/reference/kubernetes-api/authorization-resources/self-subject-access-review-v1/)
* [SelfSubjectReviews](/docs/reference/kubernetes-api/authentication-resources/self-subject-review-v1/)
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
Last modified September 11, 2025 at 9:04 PM PST: [Clarify documentation for ValidatingAdmissionPolicy and contents of CEL context (#52303) (877e7fa201)](https://github.com/kubernetes/website/commit/877e7fa201f3f7a2bcb850f1f51d74ca13b192f4)
## Related Pages

- [Using RBAC Authorization](docs-reference-access-authn-authz-rbac.md)
- [Objects In Kubernetes](docs-concepts-overview-working-with-objects.md)
- [Annotations](docs-concepts-overview-working-with-objects-annotations.md)
- [Automatic Cleanup for Finished Jobs](docs-concepts-workloads-controllers-ttlafterfinished.md)
- [Field Selectors](docs-concepts-overview-working-with-objects-field-selectors.md)