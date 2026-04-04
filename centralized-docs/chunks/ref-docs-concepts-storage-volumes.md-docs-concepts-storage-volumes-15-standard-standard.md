---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#15-standard
chunk_level: standard
chunk_type: prose
heading: Types of volumes
token_count: 417
summary: #### Caution: The `FileOrCreate` mode does **not** create the parent directory of the file. If the parent directory of the mounted file does not exist, the Pod fails to start. To ensure that this...
---

#### Caution:
The `FileOrCreate` mode does **not** create the parent directory of the file. If the parent directory
of the mounted file does not exist, the Pod fails to start. To ensure that this mode works,
you can try to mount directories and files separately, as shown in the
[`FileOrCreate` example](#hostpath-fileorcreate-example) for `hostPath`.
Some files or directories created on the underlying hosts might only be
accessible by root. You then either need to run your process as root in a
[privileged container](/docs/tasks/configure-pod-container/security-context/)
or modify the file permissions on the host to read from or write to a `hostPath` volume.
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
# mount /data/foo, but only if that directory already exists
hostPath:
path: /data/foo # directory location on host
type: Directory # this field is optional
`
```
```
`
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