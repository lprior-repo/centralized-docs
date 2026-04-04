---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#27-summary
chunk_level: summary
chunk_type: prose
heading: User accounts versus service accounts
token_count: 126
summary: The schema for the Kubernetes-specific claims within JWT tokens is not currently documented, however the relevant code area can be found in [the serviceaccount...
---

The schema for the Kubernetes-specific claims within JWT tokens is not currently documented,
however the relevant code area can be found in
[the serviceaccount package](https://github.com/kubernetes/kubernetes/blob/d8919343526597e0788a1efe133c70d9a0c07f69/pkg/serviceaccount/claims.go#L56-L68)
in the Kubernetes codebase.
You can inspect a JWT using standard JWT decoding tool. Below is an example of a JWT for the
`my-serviceaccount` ServiceAccount, bound to a Pod object named `my-pod` which is scheduled
to the Node