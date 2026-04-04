---
doc_id: ref/docs-concepts-security-controlling-access.md/docs-concepts-security-controlling-access
chunk_id: ref/docs-concepts-security-controlling-access.md/docs-concepts-security-controlling-access#11-summary
chunk_level: summary
chunk_type: prose
heading: Authorization
token_count: 99
summary: If Bob makes the following request, the request is authorized because he is allowed to read objects in the `projectCaribou` namespace: ``` `{ \"apiVersion\": \"authorization.k8s.io/v1beta1\", \"kind\":...
---

If Bob makes the following request, the request is authorized because he is
allowed to read objects in the `projectCaribou` namespace:
```
`{
"apiVersion": "authorization.k8s.io/v1beta1",
"kind": "SubjectAccessReview",
"spec": {
"resourceAttributes": {
"namespace": "projectCaribou",
"verb": "get",
"group": "unicorn.example.org",
"resource": "pods"
}
}
}
`
```