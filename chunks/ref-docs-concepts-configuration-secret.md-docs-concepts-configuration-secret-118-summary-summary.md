---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#118-summary
chunk_level: summary
chunk_type: prose
heading: Immutable Secrets
token_count: 105
summary: ``` `apiVersion: v1 kind: Secret metadata: ... data: ... immutable: true ` ``` You can also update any existing mutable Secret to make it immutable. #### Note: Once a Secret or ConfigMap is marked as...
---

```
`apiVersion: v1
kind: Secret
metadata: ...
data: ...
immutable: true
`
```
You can also update any existing mutable Secret to make it immutable.
#### Note:
Once a Secret or ConfigMap is marked as immutable, it is *not* possible to revert this change
nor to mutate the contents of the `data` field. You can only delete and recreate the Secret.
Existing Pods maintain a mount point to the deleted Secret - it is recommended to recreate
these pods.