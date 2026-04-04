---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#74-summary
chunk_level: summary
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 76
summary: To create a Secret based on this example, run: ``` `kubectl -n examplens create -f https://k8s.io/examples/secret/serviceaccount/mysecretname.yaml ` ``` To see the details for that Secret, run: ```...
---

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