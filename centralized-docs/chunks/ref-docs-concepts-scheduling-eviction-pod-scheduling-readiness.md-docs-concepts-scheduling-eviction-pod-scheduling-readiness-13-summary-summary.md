---
doc_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness
chunk_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness#13-summary
chunk_level: summary
chunk_type: prose
heading: Usage example
token_count: 121
summary: ``` `kubectl get pod test-pod ` ``` The output reveals it's in `SchedulingGated` state: ``` `NAME READY STATUS RESTARTS AGE test-pod 0/1 SchedulingGated 0 7s ` ``` You can also check its...
---

```
`kubectl get pod test-pod
`
```
The output reveals it's in `SchedulingGated` state:
```
`NAME READY STATUS RESTARTS AGE
test-pod 0/1 SchedulingGated 0 7s
`
```
You can also check its `schedulingGates` field by running:
```
`kubectl get pod test-pod -o jsonpath='{.spec.schedulingGates}'
`
```
The output is:
```
`[{"name":"example.com/foo"},{"name":"example.com/bar"}]
`
```