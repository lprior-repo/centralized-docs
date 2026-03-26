---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#30-summary
chunk_level: summary
chunk_type: prose
heading: Use cases for Kubernetes service accounts
token_count: 85
summary: * [Token Volume Projection](/docs/tasks/configure-pod-container/configure-service-account/#serviceaccount-token-volume-projection) (also recommended): In Kubernetes v1.20 and later, use the Pod...
---

* [Token Volume Projection](/docs/tasks/configure-pod-container/configure-service-account/#serviceaccount-token-volume-projection)
(also recommended): In Kubernetes v1.20 and later, use the Pod specification to
tell the kubelet to add the service account token to the Pod as a
*projected volume*. Projected tokens expire automatically, and the kubelet
rotates the token before it expires.