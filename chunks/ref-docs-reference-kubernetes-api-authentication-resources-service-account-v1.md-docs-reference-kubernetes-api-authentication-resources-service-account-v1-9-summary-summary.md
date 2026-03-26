---
doc_id: ref/docs-reference-kubernetes-api-authentication-resources-service-account-v1.md/docs-reference-kubernetes-api-authentication-resources-service-account-v1
chunk_id: ref/docs-reference-kubernetes-api-authentication-resources-service-account-v1.md/docs-reference-kubernetes-api-authentication-resources-service-account-v1#9-summary
chunk_level: summary
chunk_type: prose
heading: ServiceAccount
token_count: 120
summary: * **secrets** ([][ObjectReference](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-reference/#ObjectReference)) *Patch strategy: merge on key `name`* *Map: unique values...
---

* **secrets** ([][ObjectReference](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-reference/#ObjectReference))
*Patch strategy: merge on key `name`*
*Map: unique values on key name will be kept during a merge*
Secrets is a list of the secrets in the same namespace that pods running using this ServiceAccount are allowed to use. Pods are only limited to this list if this service account has a "kubernetes.io/enforce-mountable-secrets" annotation set to "true". The "kubernetes.io/enforce-mountable-secrets"