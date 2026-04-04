---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#3-detailed
chunk_level: detailed
chunk_type: prose
heading: User accounts versus service accounts
token_count: 907
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
#### Note:
Despite using `kubectl create -f` to create this resource, and defining it similar to
other resource types in Kubernetes, TokenReview is a special type and the kube-apiserver
does not actually persist the TokenReview object into etcd.
Hence `kubectl get tokenreview` is not a valid command.
#### Schema for service account private claims
The schema for the Kubernetes-specific claims within JWT tokens is not currently documented,
however the relevant code area can be found in
[the serviceaccount package](https://github.com/kubernetes/kubernetes/blob/d8919343526597e0788a1efe133c70d9a0c07f69/pkg/serviceaccount/claims.go#L56-L68)
in the Kubernetes codebase.
You can inspect a JWT using standard JWT decoding tool. Below is an example of a JWT for the
`my-serviceaccount` ServiceAccount, bound to a Pod object named `my-pod` which is scheduled
to the Node `my-node`, in the `my-namespace` namespace:
```
`{
"aud": [
"https://my-audience.example.com"
],
"exp": 1729605240,
"iat": 1729601640,
"iss": "https://my-cluster.example.com",
"jti": "aed34954-b33a-4142-b1ec-389d6bbb4936",
"kubernetes.io": {
"namespace": "my-namespace",
"node": {
"name": "my-node",
"uid": "646e7c5e-32d6-4d42-9dbd-e504e6cbe6b1"
},
"pod": {
"name": "my-pod",
"uid": "5e0bd49b-f040-43b0-99b7-22765a53f7f3"
},
"serviceaccount": {
"name": "my-serviceaccount",
"uid": "14ee3fa4-a7e2-420f-9f9a-dbc4507c3798"
}
},
"nbf": 1729601640,
"sub": "system:serviceaccount:my-namespace:my-serviceaccount"
}
`
```