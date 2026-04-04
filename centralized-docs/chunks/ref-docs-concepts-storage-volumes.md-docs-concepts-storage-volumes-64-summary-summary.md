---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#64-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 113
summary: # This manifest mounts C:\\Data\\foo on the host as C:\\foo, inside the # The mount into the container is read-only. apiVersion: v1 kind: Pod metadata: name: hostpath-example-windows spec: os: {...
---

# This manifest mounts C:\\Data\\foo on the host as C:\\foo, inside the
# The mount into the container is read-only.
apiVersion: v1
kind: Pod
metadata:
name: hostpath-example-windows
spec:
os: { name: windows }
nodeSelector:
kubernetes.io/os: windows
containers:
- name: example-container
image: microsoft/windowsservercore:1709
volumeMounts:
- name: example-volume
mountPath: "C:\\\\foo"
readOnly: true
volumes: