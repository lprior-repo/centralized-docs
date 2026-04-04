---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#8-detailed
chunk_level: detailed
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 972
summary: ### TokenRequest API FEATURE STATE: `Kubernetes v1.22 [stable]` You use the [TokenRequest](/docs/reference/kubernetes-api/authentication-resources/token-request-v1/) subresource of a ServiceAccount...
---

### TokenRequest API
FEATURE STATE:
`Kubernetes v1.22 [stable]`
You use the [TokenRequest](/docs/reference/kubernetes-api/authentication-resources/token-request-v1/)
subresource of a ServiceAccount to obtain a time-bound token for that ServiceAccount.
You don't need to call this to obtain an API token for use within a container, since
the kubelet sets this up for you using a *projected volume*.
If you want to use the TokenRequest API from `kubectl`, see
[Manually create an API token for a ServiceAccount](/docs/tasks/configure-pod-container/configure-service-account/#manually-create-an-api-token-for-a-serviceaccount).
The Kubernetes control plane (specifically, the ServiceAccount admission controller)
adds a projected volume to Pods, and the kubelet ensures that this volume contains a token
that lets containers authenticate as the right ServiceAccount.
(This mechanism superseded an earlier mechanism that added a volume based on a Secret,
where the Secret represented the ServiceAccount for the Pod but did not expire.)
Here's an example of how that looks for a launched Pod:
```
`...
- name: kube-api-access-&lt;random-suffix&gt;
projected:
defaultMode: 420 # decimal equivalent of octal 0644
sources:
- serviceAccountToken:
expirationSeconds: 3607
path: token
- configMap:
items:
- key: ca.crt
path: ca.crt
name: kube-root-ca.crt
- downwardAPI:
items:
- fieldRef:
apiVersion: v1
fieldPath: metadata.namespace
path: namespace
`
```
That manifest snippet defines a projected volume that combines information from three sources:
1. A `serviceAccountToken` source, that contains a token that the kubelet acquires from kube-apiserver.
The kubelet fetches time-bound tokens using the TokenRequest API. A token served for a TokenRequest expires
either when the pod is deleted or after a defined lifespan (by default, that is 1 hour).
The token is bound to the specific Pod and has the kube-apiserver as its audience.
2. A `configMap` source. The ConfigMap contains a bundle of certificate authority data. Pods can use these
certificates to make sure that they are connecting to your cluster's kube-apiserver (and not to a middlebox
or an accidentally misconfigured peer).
3. A `downwardAPI` source. This `downwardAPI` volume makes the name of the namespace containing the Pod available
to application code running inside the Pod.
Any container within the Pod that mounts this volume can access the above information.
#### Caution:
Only create long-lived API tokens if the [token request](#tokenrequest-api) mechanism
is not suitable. The token request mechanism provides time-limited tokens; because these
expire, they represent a lower risk to information security.
To create a non-expiring, persisted API token for a ServiceAccount, create a
Secret of type `kubernetes.io/service-account-token` with an annotation
referencing the ServiceAccount. The control plane then generates a long-lived token and
updates that Secret with that generated token data.
Here is a sample manifest for such a Secret:
[`secret/serviceaccount/mysecretname.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/secret/serviceaccount/mysecretname.yaml)![](/images/copycode.svg "Copy secret/serviceaccount/mysecretname.yaml to clipboard")
```
`apiVersion: v1
kind: Secret
type: kubernetes.io/service-account-token
metadata:
name: mysecretname
annotations:
kubernetes.io/service-account.name: myserviceaccount
`
```
To create a Secret based on this example, run:
```
`kubectl -n examplens create -f https://k8s.io/examples/secret/serviceaccount/mysecretname.yaml
`
```
To see the details for that Secret, run:
```
`kubectl -n examplens describe secret mysecretname
`
```
The output is similar to:
```
`Name: mysecretname
Namespace: examplens
Labels: &lt;none&gt;
Annotations: kubernetes.io/service-account.name=myserviceaccount
kubernetes.io/service-account.uid=8a85c4c4-8483-11e9-bc42-526af7764f64
Type: kubernetes.io/service-account-token
Data
====
ca.crt: 1362 bytes
namespace: 9 bytes
token: ...
`
```
If you launch a new Pod into the `examplens` namespace, it can use the `myserviceaccount`
service-account-token Secret that you just created.