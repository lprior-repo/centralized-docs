---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#96-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 116
summary: A `portworxVolume` can be dynamically created through Kubernetes, or it can also be pre-provisioned and referenced inside a Pod. Here is an example Pod referencing a pre-provisioned Portworx volume:...
---

A `portworxVolume` can be dynamically created through Kubernetes, or it can also
be pre-provisioned and referenced inside a Pod.
Here is an example Pod referencing a pre-provisioned Portworx volume:
```
`apiVersion: v1
kind: Pod
metadata:
name: test-portworx-volume-pod
spec:
containers:
- image: registry.k8s.io/test-webserver
name: test-container
volumeMounts:
- mountPath: /mnt
name: pxvol
volumes:
- name: pxvol