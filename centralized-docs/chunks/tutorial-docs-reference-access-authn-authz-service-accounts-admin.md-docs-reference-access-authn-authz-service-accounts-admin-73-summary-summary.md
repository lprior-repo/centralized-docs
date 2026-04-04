---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#73-summary
chunk_level: summary
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 126
summary: updates that Secret with that generated token data. Here is a sample manifest for such a Secret: [`secret/serviceaccount/mysecretname.yaml`...
---

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