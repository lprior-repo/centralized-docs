---
doc_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics
chunk_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics#27-summary
chunk_level: summary
chunk_type: table
heading: Example Scenarios
token_count: 102
summary: #### Observing I/O Pressure **Using the Summary API:** You will see the `some` PSI metrics for I/O increase as the Pod continuously writes to disk. ``` `# Replace &lt;node-name&gt; with the name of a...
---

#### Observing I/O Pressure
**Using the Summary API:**
You will see the `some` PSI metrics for I/O increase as the Pod continuously writes to disk.
```
`# Replace &lt;node-name&gt; with the name of a node in your cluster
kubectl get --raw "/api/v1/nodes/&lt;node-name&gt;/proxy/stats/summary" | jq '.pods[] | select(.podRef.name | contains("io-pressure-pod"))'
`
```