---
doc_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness
chunk_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness#4-standard
chunk_level: standard
chunk_type: prose
heading: Observability
token_count: 228
summary: You can check if the `schedulingGates` is cleared by running: ``` `kubectl get pod test-pod -o jsonpath='{.spec.schedulingGates}' ` ``` The output is expected to be empty. And you can check its...
---

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
Given the test-pod doesn't request any CPU/memory resources, it's expected that this Pod's state get
transited from previous `SchedulingGated` to `Running`:
```
`NAME READY STATUS RESTARTS AGE IP NODE
test-pod 1/1 Running 0 15s 10.0.0.4 node-2
`
```
## Observability
The metric `scheduler\_pending\_pods` comes with a new label `"gated"` to distinguish whether a Pod
has been tried scheduling but claimed as unschedulable, or explicitly marked as not ready for
scheduling. You can use `scheduler\_pending\_pods{queue="gated"}` to check the metric result.