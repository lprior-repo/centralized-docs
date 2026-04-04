---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#6-detailed
chunk_level: detailed
chunk_type: prose
heading: Types of volumes
token_count: 573
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
# mount C:\\Data\\foo from the host, but only if that directory already exists
- name: example-volume
hostPath:
path: "C:\\\\Data\\\\foo" # directory location on host
type: Directory # this field is optional
`
```
#### hostPath FileOrCreate configuration example
The following manifest defines a Pod that mounts `/var/local/aaa`
inside the single container in the Pod. If the node does not
already have a path `/var/local/aaa`, the kubelet creates
it as a directory and then mounts it into the Pod.
If `/var/local/aaa` already exists but is not a directory,
the Pod fails. Additionally, the kubelet attempts to make
a file named `/var/local/aaa/1.txt` inside that directory
(as seen from the host); if something already exists at
that path and isn't a regular file, the Pod fails.
Here's the example manifest:
```
`apiVersion: v1
kind: Pod
metadata:
name: test-webserver
spec:
os: { name: linux }
nodeSelector:
kubernetes.io/os: linux
containers:
- name: test-webserver
image: registry.k8s.io/test-webserver:latest
volumeMounts:
- mountPath: /var/local/aaa
name: mydir
- mountPath: /var/local/aaa/1.txt
name: myfile
volumes:
- name: mydir
hostPath: