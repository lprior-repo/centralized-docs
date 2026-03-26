---
doc_id: ref/docs-concepts-cluster-administration-kube-state-metrics.md/docs-concepts-cluster-administration-kube-state-metrics
chunk_id: ref/docs-concepts-cluster-administration-kube-state-metrics.md/docs-concepts-cluster-administration-kube-state-metrics#7-summary
chunk_level: summary
chunk_type: prose
heading: Example: alerting based on from kube-state-metrics
token_count: 109
summary: ``` `groups: - name: Pod state rules: - alert: PodsBlockedInTerminatingState expr: count(kube\_pod\_deletion\_timestamp) by (namespace, pod) \* count(kube\_pod\_status\_reason{reason=\"NodeLost\"} ==...
---

```
`groups:
- name: Pod state
rules:
- alert: PodsBlockedInTerminatingState
expr: count(kube\_pod\_deletion\_timestamp) by (namespace, pod) \* count(kube\_pod\_status\_reason{reason="NodeLost"} == 0) by (namespace, pod) &gt; 0
for: 5m
labels:
severity: page
annotations:
summary: Pod {{$labels.namespace}}/{{$labels.pod}} blocked in Terminating state.
`
```