---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#146-summary
chunk_level: summary
chunk_type: prose
heading: Read-only mounts
token_count: 122
summary: * The OCI-level container runtime supports recursive read-only mounts. It will fail if any of these is not true. * `IfPossible`: attempts to apply `Enabled`, and falls back to `Disabled` if the...
---

* The OCI-level container runtime supports recursive read-only mounts.
It will fail if any of these is not true.
* `IfPossible`: attempts to apply `Enabled`, and falls back to `Disabled`
if the feature is not supported by the kernel or the runtime class.
Example:
[`storage/rro.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/storage/rro.yaml)![](/images/copycode.svg "Copy storage/rro.yaml to clipboard")
```
`apiVersion: v1
kind: Pod
metadata:
name: rro
spec:
volumes: