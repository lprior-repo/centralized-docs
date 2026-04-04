---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#74-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 117
summary: The volume is resolved at Pod startup, depending on which `pullPolicy` value is provided: `Always`The kubelet always attempts to pull the reference. If the pull fails, the kubelet sets the Pod to...
---

The volume is resolved at Pod startup, depending on which `pullPolicy` value is
provided:
`Always`The kubelet always attempts to pull the reference. If the pull fails,
the kubelet sets the Pod to `Failed`.`Never`The kubelet never pulls the reference and only uses a local image or artifact.
The Pod becomes `Failed` if any layers of the image aren't already present locally,
or if the manifest for that image isn't already cached.`IfNotPresent`The kubelet pulls if the reference isn't already present on disk. The Pod becomes