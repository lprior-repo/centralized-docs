---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#77-summary
chunk_level: summary
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 100
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