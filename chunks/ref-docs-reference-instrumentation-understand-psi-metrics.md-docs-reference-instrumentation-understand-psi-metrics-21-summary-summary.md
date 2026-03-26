---
doc_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics
chunk_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics#21-summary
chunk_level: summary
chunk_type: table
heading: Example Scenarios
token_count: 110
summary: #### Observing Memory Pressure **Using the Summary API:** In the summary output, you will observe an increase in the `full` PSI metrics for memory, indicating that the system is under significant...
---

#### Observing Memory Pressure
**Using the Summary API:**
In the summary output, you will observe an increase in the `full` PSI metrics for memory, indicating that the system is under significant memory pressure.
```
`# Replace &lt;node-name&gt; with the name of a node in your cluster
kubectl get --raw "/api/v1/nodes/&lt;node-name&gt;/proxy/stats/summary" | jq '.pods[] | select(.podRef.name | contains("memory-pressure-pod"))'
`
```