---
doc_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods
chunk_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods#26-summary
chunk_level: summary
chunk_type: prose
heading: Diagnosing the problem
token_count: 118
summary: #### My service is missing endpoints If you are missing endpoints, try listing pods using the labels that Service uses. Imagine that you have a Service where the labels are: ``` `... spec: -...
---

#### My service is missing endpoints
If you are missing endpoints, try listing pods using the labels that Service uses.
Imagine that you have a Service where the labels are:
```
`...
spec:
- selector:
name: nginx
type: frontend
`
```
You can use:
```
`kubectl get pods --selector=name=nginx,type=frontend
`
```
to list pods that match this selector. Verify that the list matches the Pods that you expect to provide your Service.
Verify that the pod's `containerPort` matches up with the Service's `targetPort`