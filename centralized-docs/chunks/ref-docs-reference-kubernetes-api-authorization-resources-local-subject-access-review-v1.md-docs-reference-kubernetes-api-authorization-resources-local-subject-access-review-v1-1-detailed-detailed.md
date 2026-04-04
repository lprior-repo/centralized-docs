---
doc_id: ref/docs-reference-kubernetes-api-authorization-resources-local-subject-access-review-v1.md/docs-reference-kubernetes-api-authorization-resources-local-subject-access-review-v1
chunk_id: ref/docs-reference-kubernetes-api-authorization-resources-local-subject-access-review-v1.md/docs-reference-kubernetes-api-authorization-resources-local-subject-access-review-v1#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 989
summary: # LocalSubjectAccessReview LocalSubjectAccessReview checks whether or not a user or group can perform an action in a given namespace. `apiVersion: authorization.k8s.io/v1` `import...
---

# LocalSubjectAccessReview
LocalSubjectAccessReview checks whether or not a user or group can perform an action in a given namespace.
`apiVersion: authorization.k8s.io/v1`
`import "k8s.io/api/authorization/v1"`
## LocalSubjectAccessReview
LocalSubjectAccessReview checks whether or not a user or group can perform an action in a given namespace. Having a namespace scoped resource makes it much easier to grant namespace scoped policy that includes permissions checking.
* **apiVersion**: authorization.k8s.io/v1
* **kind**: LocalSubjectAccessReview
* **metadata** ([ObjectMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#ObjectMeta))
Standard list metadata. More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata)
* **spec** ([SubjectAccessReviewSpec](https://kubernetes.io/docs/reference/kubernetes-api/authorization-resources/subject-access-review-v1/#SubjectAccessReviewSpec)), required
Spec holds information about the request being evaluated. spec.namespace must be equal to the namespace you made the request against. If empty, it is defaulted.
* **status** ([SubjectAccessReviewStatus](https://kubernetes.io/docs/reference/kubernetes-api/authorization-resources/subject-access-review-v1/#SubjectAccessReviewStatus))
Status is filled in by the server and indicates whether the request is allowed or not
#### Parameters
* **namespace** (*in path*): string, required
[namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace)
* **body**: [LocalSubjectAccessReview](https://kubernetes.io/docs/reference/kubernetes-api/authorization-resources/local-subject-access-review-v1/#LocalSubjectAccessReview), required
* **dryRun** (*in query*): string
[dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldManager** (*in query*): string
[fieldManager](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldManager)
* **fieldValidation** (*in query*): string
[fieldValidation](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldValidation)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
#### Response
200 ([LocalSubjectAccessReview](https://kubernetes.io/docs/reference/kubernetes-api/authorization-resources/local-subject-access-review-v1/#LocalSubjectAccessReview)): OK
201 ([LocalSubjectAccessReview](https://kubernetes.io/docs/reference/kubernetes-api/authorization-resources/local-subject-access-review-v1/#LocalSubjectAccessReview)): Created
202 ([LocalSubjectAccessReview](https://kubernetes.io/docs/reference/kubernetes-api/authorization-resources/local-subject-access-review-v1/#LocalSubjectAccessReview)): Accepted
401: Unauthorized
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
Last modified August 28, 2024 at 6:01 PM PST: [Update generated API reference for v1.31 (8ba98c79c1)](https://github.com/kubernetes/website/commit/8ba98c79c169bb070416a685db63074847399df5)
This page is automatically generated.
If you plan to report an issue with this page, mention that the page is auto-generated in your issue description. The fix may need to happen elsewhere in the Kubernetes project.
## Related Pages

- [Binding](docs-reference-kubernetes-api-workload-resources-binding-v1.md)
- [HorizontalPodAutoscaler](docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)
- [Workload v1alpha1](docs-reference-kubernetes-api-workload-resources-workload-v1alpha1.md)
- [LeaseCandidate v1beta1](docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1.md)
- [Secret](docs-reference-kubernetes-api-config-and-storage-resources-secret-v1.md)