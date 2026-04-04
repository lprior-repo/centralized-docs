---
doc_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness
chunk_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness#15-summary
chunk_level: summary
chunk_type: prose
heading: Usage example
token_count: 120
summary: ``` `apiVersion: v1 kind: Pod metadata: name: test-pod spec: containers: - name: pause image: registry.k8s.io/pause:3.6 ` ``` You can check if the `schedulingGates` is cleared by running: ```...
---

```
`apiVersion: v1
kind: Pod
metadata:
name: test-pod
spec:
containers:
- name: pause
image: registry.k8s.io/pause:3.6
`
```
You can check if the `schedulingGates` is cleared by running:
```
`kubectl get pod test-pod -o jsonpath='{.spec.schedulingGates}'
`
```
The output is expected to be empty. And you can check its latest status by running:
```
`kubectl get pod test-pod -o wide
`
```