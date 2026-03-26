---
doc_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics
chunk_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics#20-summary
chunk_level: summary
chunk_type: prose
heading: Example Scenarios
token_count: 38
summary: resources: limits: memory: \"200M\" requests: memory: \"200M\" ` ``` Apply it to your cluster: `kubectl apply -f memory-pressure-pod.yaml`
---

resources:
limits:
memory: "200M"
requests:
memory: "200M"
`
```
Apply it to your cluster: `kubectl apply -f memory-pressure-pod.yaml`