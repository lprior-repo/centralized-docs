---
doc_id: tutorial/docs-tasks-configure-pod-container-image-volumes.md/docs-tasks-configure-pod-container-image-volumes
chunk_id: tutorial/docs-tasks-configure-pod-container-image-volumes.md/docs-tasks-configure-pod-container-image-volumes#8-summary
chunk_level: summary
chunk_type: prose
heading: Use `subPath` (or `subPathExpr`)
token_count: 128
summary: ``` `apiVersion: v1 kind: Pod metadata: name: image-volume spec: containers: - name: shell command: [\"sleep\", \"infinity\"] image: debian volumeMounts: - name: volume mountPath: /volume subPath: dir...
---

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