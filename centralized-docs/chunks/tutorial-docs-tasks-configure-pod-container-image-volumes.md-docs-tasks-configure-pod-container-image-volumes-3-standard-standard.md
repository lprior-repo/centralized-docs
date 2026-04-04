---
doc_id: tutorial/docs-tasks-configure-pod-container-image-volumes.md/docs-tasks-configure-pod-container-image-volumes
chunk_id: tutorial/docs-tasks-configure-pod-container-image-volumes.md/docs-tasks-configure-pod-container-image-volumes#3-standard
chunk_level: standard
chunk_type: code
heading: Use `subPath` (or `subPathExpr`)
token_count: 444
summary: 1. Create the pod on your cluster: ``` `kubectl apply -f https://k8s.io/examples/pods/image-volumes.yaml ` ``` 2. Attach to the container: ``` `kubectl exec image-volume -it -- bash ` ``` 3. Check...
---

1. Create the pod on your cluster:
```
`kubectl apply -f https://k8s.io/examples/pods/image-volumes.yaml
`
```
2. Attach to the container:
```
`kubectl exec image-volume -it -- bash
`
```
3. Check the content of a file in the volume:
```
`cat /volume/dir/file
`
```
The output is similar to:
```
`1
`
```
You can also check another file in a different path:
```
`cat /volume/file
`
```
The output is similar to:
```
`2
`
```
## Use `subPath` (or `subPathExpr`)
It is possible to utilize
[`subPath`](/docs/concepts/storage/volumes/#using-subpath) or
[`subPathExpr`](/docs/concepts/storage/volumes/#using-subpath-expanded-environment)
from Kubernetes v1.33 when using the image volume feature.
[`pods/image-volumes-subpath.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/image-volumes-subpath.yaml)![](/images/copycode.svg "Copy pods/image-volumes-subpath.yaml to clipboard")
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
subPath: dir
volumes:
- name: volume
image:
reference: quay.io/crio/artifact:v2
pullPolicy: IfNotPresent
`
```
1. Create the pod on your cluster:
```
`kubectl apply -f https://k8s.io/examples/pods/image-volumes-subpath.yaml
`
```
2. Attach to the container:
```
`kubectl exec image-volume -it -- bash
`
```
3. Check the content of the file from the `dir` sub path in the volume:
```
`cat /volume/file
`
```
The output is similar to:
```
`1
`
```