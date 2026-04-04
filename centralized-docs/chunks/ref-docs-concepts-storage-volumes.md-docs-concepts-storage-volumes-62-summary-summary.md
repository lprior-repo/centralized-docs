---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#62-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 111
summary: # This manifest mounts /data/foo on the host as /foo inside the # The mount into the container is read-only. apiVersion: v1 kind: Pod metadata: name: hostpath-example-linux spec: os: { name: linux }...
---

# This manifest mounts /data/foo on the host as /foo inside the
# The mount into the container is read-only.
apiVersion: v1
kind: Pod
metadata:
name: hostpath-example-linux
spec:
os: { name: linux }
nodeSelector:
kubernetes.io/os: linux
containers:
- name: example-container
image: registry.k8s.io/test-webserver
volumeMounts:
- mountPath: /foo
name: example-volume
readOnly: true
volumes:
- name: example-volume