---
doc_id: ref/docs-reference-kubernetes-api-authorization-resources-local-subject-access-review-v1.md/docs-reference-kubernetes-api-authorization-resources-local-subject-access-review-v1
chunk_id: ref/docs-reference-kubernetes-api-authorization-resources-local-subject-access-review-v1.md/docs-reference-kubernetes-api-authorization-resources-local-subject-access-review-v1#4-summary
chunk_level: summary
chunk_type: prose
heading: LocalSubjectAccessReview
token_count: 116
summary: * **spec** ([SubjectAccessReviewSpec](https://kubernetes.io/docs/reference/kubernetes-api/authorization-resources/subject-access-review-v1/#SubjectAccessReviewSpec)), required Spec holds information...
---

* **spec** ([SubjectAccessReviewSpec](https://kubernetes.io/docs/reference/kubernetes-api/authorization-resources/subject-access-review-v1/#SubjectAccessReviewSpec)), required
Spec holds information about the request being evaluated. spec.namespace must be equal to the namespace you made the request against. If empty, it is defaulted.
* **status** ([SubjectAccessReviewStatus](https://kubernetes.io/docs/reference/kubernetes-api/authorization-resources/subject-access-review-v1/#SubjectAccessReviewStatus))
Status is filled in by the server and indicates whether the request is allowed or not