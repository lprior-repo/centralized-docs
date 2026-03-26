---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#37-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 118
summary: ### Opaque Secrets `Opaque` is the default Secret type if you don't explicitly specify a type in a Secret manifest. When you create a Secret using `kubectl`, you must use the `generic` subcommand to...
---

### Opaque Secrets
`Opaque` is the default Secret type if you don't explicitly specify a type in
a Secret manifest. When you create a Secret using `kubectl`, you must use the
`generic` subcommand to indicate an `Opaque` Secret type. For example, the
following command creates an empty Secret of type `Opaque`:
```
`kubectl create secret generic empty-secret
kubectl get secret empty-secret
`
```
The output looks like:
```
`NAME TYPE DATA AGE
empty-secret Opaque 0 2m6s
`
```