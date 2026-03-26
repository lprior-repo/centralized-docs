---
doc_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics
chunk_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics#23-summary
chunk_level: summary
chunk_type: prose
heading: Example Scenarios
token_count: 49
summary: In the output, you will observe an increasing value for the metric, indicating that the system is under significant memory pressure. #### Cleanup Clean up the Pod when you are finished: ``` `kubectl...
---

In the output, you will observe an increasing value for the metric, indicating that the system is under significant memory pressure.
#### Cleanup
Clean up the Pod when you are finished:
```
`kubectl delete pod memory-pressure-pod
`
```