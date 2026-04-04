---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#17-standard
chunk_level: standard
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 407
summary: ### Delete/invalidate a long-lived/legacy ServiceAccount token If you know the name of the Secret that contains the token you want to remove: ``` `kubectl delete secret name-of-secret ` ```...
---

### Delete/invalidate a long-lived/legacy ServiceAccount token
If you know the name of the Secret that contains the token you want to remove:
```
`kubectl delete secret name-of-secret
`
```
Otherwise, first find the Secret for the ServiceAccount.
```
`# This assumes that you already have a namespace named 'examplens'
kubectl -n examplens get serviceaccount/example-automated-thing -o yaml
`
```
The output is similar to:
```
`apiVersion: v1
kind: ServiceAccount
metadata:
annotations:
kubectl.kubernetes.io/last-applied-configuration: |
{"apiVersion":"v1","kind":"ServiceAccount","metadata":{"annotations":{},"name":"example-automated-thing","namespace":"examplens"}}
creationTimestamp: "2019-07-21T07:07:07Z"
name: example-automated-thing
namespace: examplens
resourceVersion: "777"
selfLink: /api/v1/namespaces/examplens/serviceaccounts/example-automated-thing
uid: f23fd170-66f2-4697-b049-e1e266b7f835
secrets:
- name: example-automated-thing-token-zyxwv
`
```
Then, delete the Secret you now know the name of:
```
`kubectl -n examplens delete secret/example-automated-thing-token-zyxwv
`
```
### Delete/invalidate a short-lived ServiceAccount token
Short lived ServiceAccount tokens automatically expire after the time-limit
specified during their creation. There is no central record of tokens issued,
so there is no way to revoke individual tokens.
If you have to revoke a short-lived token before its expiration, you
can delete and re-create the ServiceAccount it is associated to. This will
change its UID and hence invalidate **all** ServiceAccount tokens that were
created for it.