---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#16-standard
chunk_level: standard
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 491
summary: #### Caution: Only create long-lived API tokens if the [token request](#tokenrequest-api) mechanism is not suitable. The token request mechanism provides time-limited tokens; because these expire,...
---

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
#### Caution:
Do not reference manually created Secrets in the `secrets` field of a
ServiceAccount. Or the manually created Secrets will be cleaned if it is not used for a long
time. Please refer to [auto-generated legacy ServiceAccount token clean up](#auto-generated-legacy-serviceaccount-token-clean-up).