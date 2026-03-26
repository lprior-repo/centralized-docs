---
doc_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics
chunk_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics#14-summary
chunk_level: summary
chunk_type: table
heading: Example Scenarios
token_count: 118
summary: #### Observing CPU Pressure After the Pod is running, you can observe the CPU pressure through either the Summary API or the Prometheus metrics endpoint. **Using the Summary API:** Watch the summary...
---

#### Observing CPU Pressure
After the Pod is running, you can observe the CPU pressure through either the Summary API or the Prometheus metrics endpoint.
**Using the Summary API:**
Watch the summary stats for your node. In a separate terminal, run:
```
`# Replace &lt;node-name&gt; with the name of a node in your cluster
kubectl get --raw "/api/v1/nodes/&lt;node-name&gt;/proxy/stats/summary" | jq '.pods[] | select(.podRef.name | contains("cpu-pressure-pod"))'
`
```