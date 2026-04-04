---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#22-summary
chunk_level: summary
chunk_type: prose
heading: User accounts versus service accounts
token_count: 114
summary: `apiVersion: authentication.k8s.io/v1 kind: TokenReview metadata: creationTimestamp: null spec: token: &lt;token&gt; status: audiences: - https://kubernetes.default.svc.cluster.local authenticated:...
---

`apiVersion: authentication.k8s.io/v1
kind: TokenReview
metadata:
creationTimestamp: null
spec:
token: &lt;token&gt;
status:
audiences:
- https://kubernetes.default.svc.cluster.local
authenticated: true
user:
extra:
authentication.kubernetes.io/credential-id:
- JTI=7ee52be0-9045-4653-aa5e-0da57b8dccdc
authentication.kubernetes.io/node-name:
- kind-control-plane
authentication.kubernetes.io/node-uid: