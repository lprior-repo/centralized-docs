---
doc_id: tutorial/docs-tasks-configure-pod-container-image-volumes.md/docs-tasks-configure-pod-container-image-volumes
chunk_id: tutorial/docs-tasks-configure-pod-container-image-volumes.md/docs-tasks-configure-pod-container-image-volumes#6-summary
chunk_level: summary
chunk_type: code
heading: Before you begin
token_count: 119
summary: ``` `kubectl apply -f https://k8s.io/examples/pods/image-volumes.yaml ` ``` 2. Attach to the container: ``` `kubectl exec image-volume -it -- bash ` ``` 3. Check the content of a file in the volume:...
---

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