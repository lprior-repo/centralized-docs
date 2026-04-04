---
doc_id: ref/docs-concepts-security-controlling-access.md/docs-concepts-security-controlling-access
chunk_id: ref/docs-concepts-security-controlling-access.md/docs-concepts-security-controlling-access#10-summary
chunk_level: summary
chunk_type: prose
heading: Authorization
token_count: 90
summary: ``` `{ \"apiVersion\": \"abac.authorization.kubernetes.io/v1beta1\", \"kind\": \"Policy\", \"spec\": { \"user\": \"bob\", \"namespace\": \"projectCaribou\", \"resource\": \"pods\", \"readonly\": true } } ` ``` If Bob makes...
---

```
`{
"apiVersion": "abac.authorization.kubernetes.io/v1beta1",
"kind": "Policy",
"spec": {
"user": "bob",
"namespace": "projectCaribou",
"resource": "pods",
"readonly": true
}
}
`
```
If Bob makes the following request, the request is authorized because he is
allowed to read objects in the `projectCaribou` namespace: