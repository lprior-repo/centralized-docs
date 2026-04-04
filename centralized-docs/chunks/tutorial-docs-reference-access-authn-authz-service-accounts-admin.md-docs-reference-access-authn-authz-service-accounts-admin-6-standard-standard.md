---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#6-standard
chunk_level: standard
chunk_type: prose
heading: User accounts versus service accounts
token_count: 457
summary: #### Note: Despite using `kubectl create -f` to create this resource, and defining it similar to other resource types in Kubernetes, TokenReview is a special type and the kube-apiserver does not...
---

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