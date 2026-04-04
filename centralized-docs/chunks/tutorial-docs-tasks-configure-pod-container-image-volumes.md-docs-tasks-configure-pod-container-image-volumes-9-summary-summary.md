---
doc_id: tutorial/docs-tasks-configure-pod-container-image-volumes.md/docs-tasks-configure-pod-container-image-volumes
chunk_id: tutorial/docs-tasks-configure-pod-container-image-volumes.md/docs-tasks-configure-pod-container-image-volumes#9-summary
chunk_level: summary
chunk_type: prose
heading: Use `subPath` (or `subPathExpr`)
token_count: 91
summary: ``` `kubectl apply -f https://k8s.io/examples/pods/image-volumes-subpath.yaml ` ``` 2. Attach to the container: ``` `kubectl exec image-volume -it -- bash ` ``` 3. Check the content of the file from...
---

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