---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#8-standard
chunk_level: standard
chunk_type: prose
heading: Types of Secret
token_count: 150
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
The `DATA` column shows the number of data items stored in the Secret.
In this case, `0` means you have created an empty Secret.