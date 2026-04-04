---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#36-standard
chunk_level: standard
chunk_type: prose
heading: Read-only mounts
token_count: 319
summary: * `Enabled`: makes the mount recursively read-only. Needs all the following requirements to be satisfied: * `readOnly` is set to `true` * `mountPropagation` is unset, or set to `None` * The host is...
---

* `Enabled`: makes the mount recursively read-only.
Needs all the following requirements to be satisfied:
* `readOnly` is set to `true`
* `mountPropagation` is unset, or set to `None`
* The host is running with Linux kernel v5.12 or later
* The [CRI-level](/docs/concepts/architecture/cri) container runtime supports recursive read-only mounts
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
# /mnt-ro/tmpfs is writable
- name: mnt
mountPath: /mnt-ro
readOnly: true