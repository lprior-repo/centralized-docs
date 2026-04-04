---
doc_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness
chunk_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness#12-summary
chunk_level: summary
chunk_type: prose
heading: Usage example
token_count: 106
summary: ``` `apiVersion: v1 kind: Pod metadata: name: test-pod spec: schedulingGates: - name: example.com/foo - name: example.com/bar containers: - name: pause image: registry.k8s.io/pause:3.6 ` ``` After...
---

```
`apiVersion: v1
kind: Pod
metadata:
name: test-pod
spec:
schedulingGates:
- name: example.com/foo
- name: example.com/bar
containers:
- name: pause
image: registry.k8s.io/pause:3.6
`
```
After the Pod's creation, you can check its state using:
```
`kubectl get pod test-pod
`
```
The output reveals it's in `SchedulingGated` state: