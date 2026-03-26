---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#16-summary
chunk_level: summary
chunk_type: prose
heading: Use cases for Kubernetes service accounts
token_count: 120
summary: * [Authenticating to a private image registry using an `imagePullSecret`](/docs/tasks/configure-pod-container/configure-service-account/#add-imagepullsecrets-to-a-service-account). * An external...
---

* [Authenticating to a private image registry using an `imagePullSecret`](/docs/tasks/configure-pod-container/configure-service-account/#add-imagepullsecrets-to-a-service-account).
* An external service needs to communicate with the Kubernetes API server. For
example, authenticating to the cluster as part of a CI/CD pipeline.
* You use third-party security software in your cluster that relies on the
ServiceAccount identity of different Pods to group those Pods into different
contexts.## How to use service accounts
To use a Kubernetes service account, you do the following: