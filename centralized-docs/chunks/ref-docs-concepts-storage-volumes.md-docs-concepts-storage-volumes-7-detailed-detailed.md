---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#7-detailed
chunk_level: detailed
chunk_type: prose
heading: Types of volumes
token_count: 242
summary: # Ensure the file directory is created. path: /var/local/aaa type: DirectoryOrCreate - name: myfile hostPath: path: /var/local/aaa/1.txt type: FileOrCreate ` ``` ### image FEATURE STATE: `Kubernetes...
---

# Ensure the file directory is created.
path: /var/local/aaa
type: DirectoryOrCreate
- name: myfile
hostPath:
path: /var/local/aaa/1.txt
type: FileOrCreate
`
```
### image
FEATURE STATE:
`Kubernetes v1.35 [beta]`(enabled by default)
An `image` volume source represents an OCI object (a container image or
artifact) which is available on the kubelet's host machine.
An example of using the `image` volume source is:
[`pods/image-volumes.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/image-volumes.yaml)![](/images/copycode.svg "Copy pods/image-volumes.yaml to clipboard")
```
`apiVersion: v1
kind: Pod
metadata:
name: image-volume
spec:
containers:
- name: shell
command: ["sleep", "infinity"]
image: debian
volumeMounts:
- name: volume
mountPath: /volume
volumes:
- name: volume
image:
reference: quay.io/crio/artifact:v2
pullPolicy: IfNotPresent
`