---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#75-summary
chunk_level: summary
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 123
summary: ``` `kubectl -n examplens describe secret mysecretname ` ``` The output is similar to: ``` `Name: mysecretname Namespace: examplens Labels: &lt;none&gt; Annotations:...
---

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