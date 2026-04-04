---
doc_id: ref/docs-reference-kubernetes-api-authorization-resources-local-subject-access-review-v1.md/docs-reference-kubernetes-api-authorization-resources-local-subject-access-review-v1
chunk_id: ref/docs-reference-kubernetes-api-authorization-resources-local-subject-access-review-v1.md/docs-reference-kubernetes-api-authorization-resources-local-subject-access-review-v1#2-standard
chunk_level: standard
chunk_type: prose
heading: LocalSubjectAccessReview
token_count: 476
summary: ## LocalSubjectAccessReview LocalSubjectAccessReview checks whether or not a user or group can perform an action in a given namespace. Having a namespace scoped resource makes it much easier to grant...
---

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