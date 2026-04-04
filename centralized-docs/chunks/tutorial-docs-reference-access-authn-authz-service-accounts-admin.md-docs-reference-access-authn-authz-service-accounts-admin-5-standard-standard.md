---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#5-standard
chunk_level: standard
chunk_type: prose
heading: User accounts versus service accounts
token_count: 449
summary: ### Verifying and inspecting private claims The TokenReview API can be used to verify and extract private claims from a token: 1. First, assume you have a pod named `test-pod` and a service account...
---

### Verifying and inspecting private claims
The TokenReview API can be used to verify and extract private claims from a token:
1. First, assume you have a pod named `test-pod` and a service account named `my-sa`.
2. Create a token that is bound to this Pod:
```
`kubectl create token my-sa --bound-object-kind="Pod" --bound-object-name="test-pod"
`
```
3. Copy this token into a new file named `tokenreview.yaml`:
```
`apiVersion: authentication.k8s.io/v1
kind: TokenReview
spec:
token: &lt;token from step 2&gt;
`
```
4. Submit this resource to the apiserver for review:
```
`# use '-o yaml' to inspect the output
kubectl create -o yaml -f tokenreview.yaml
`
```
You should see an output like below:
```
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
- 497e9d9a-47aa-4930-b0f6-9f2fb574c8c6
authentication.kubernetes.io/pod-name:
- test-pod
authentication.kubernetes.io/pod-uid:
- e87dbbd6-3d7e-45db-aafb-72b24627dff5
groups:
- system:serviceaccounts
- system:serviceaccounts:default
- system:authenticated
uid: f8b4161b-2e2b-11e9-86b7-2afc33b31a7e
username: system:serviceaccount:default:my-sa
`
```