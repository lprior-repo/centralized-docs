---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#38-summary
chunk_level: summary
chunk_type: prose
heading: Bound service account token volume mechanism
token_count: 118
summary: 1. A `serviceAccountToken` source, that contains a token that the kubelet acquires from kube-apiserver. The kubelet fetches time-bound tokens using the TokenRequest API. A token served for a...
---

1. A `serviceAccountToken` source, that contains a token that the kubelet acquires from kube-apiserver.
The kubelet fetches time-bound tokens using the TokenRequest API. A token served for a TokenRequest expires
either when the pod is deleted or after a defined lifespan (by default, that is 1 hour).
The kubelet also refreshes that token before the token expires.
The token is bound to the specific Pod and has the kube-apiserver as its audience.
This mechanism superseded an earlier mechanism that added a volume based on a Secret,