---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#80-summary
chunk_level: summary
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 113
summary: resourceVersion: \"777\" selfLink: /api/v1/namespaces/examplens/serviceaccounts/example-automated-thing uid: f23fd170-66f2-4697-b049-e1e266b7f835 secrets: - name: example-automated-thing-token-zyxwv `...
---

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