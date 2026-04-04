---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#136-summary
chunk_level: summary
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 121
summary: volume types. The Kubernetes project recommends only using mount propagation with `hostPath` or memory-backed `emptyDir` volumes. See [Kubernetes issue...
---

volume types. The Kubernetes project recommends only using mount propagation with `hostPath`
or memory-backed `emptyDir` volumes. See
[Kubernetes issue #95049](https://github.com/kubernetes/kubernetes/issues/95049)
for more context.
Mount propagation allows for sharing volumes mounted by a container to
other containers in the same Pod, or even to other Pods on the same node.
Mount propagation of a volume is controlled by the `mountPropagation` field
in `containers[\*].volumeMounts`. Its values are:
* `None` - This volume mount will not receive any subsequent mounts