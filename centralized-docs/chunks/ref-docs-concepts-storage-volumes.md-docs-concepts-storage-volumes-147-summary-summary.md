---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#147-summary
chunk_level: summary
chunk_type: prose
heading: Read-only mounts
token_count: 111
summary: `apiVersion: v1 kind: Pod metadata: name: rro spec: volumes: - name: mnt hostPath: # tmpfs is mounted on /mnt/tmpfs path: /mnt containers: - name: busybox image: busybox args: [\"sleep\", \"infinity\"]...
---

`apiVersion: v1
kind: Pod
metadata:
name: rro
spec:
volumes:
- name: mnt
hostPath:
# tmpfs is mounted on /mnt/tmpfs
path: /mnt
containers:
- name: busybox
image: busybox
args: ["sleep", "infinity"]
volumeMounts:
# /mnt-rro/tmpfs is not writable
- name: mnt
mountPath: /mnt-rro
readOnly: true
mountPropagation: None
recursiveReadOnly: Enabled