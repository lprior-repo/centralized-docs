---
doc_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics
chunk_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics#28-summary
chunk_level: summary
chunk_type: prose
heading: Example Scenarios
token_count: 122
summary: **Using the Prometheus metrics endpoint:** Query the `/metrics/cadvisor` endpoint to see the `container\_pressure\_io\_waiting\_seconds\_total` metric. ``` `# Replace &lt;node-name&gt; with the name...
---

**Using the Prometheus metrics endpoint:**
Query the `/metrics/cadvisor` endpoint to see the `container\_pressure\_io\_waiting\_seconds\_total` metric.
```
`# Replace &lt;node-name&gt; with the name of the node where the pod is running
kubectl get --raw "/api/v1/nodes/&lt;node-name&gt;/proxy/metrics/cadvisor" | \\
grep 'container\_pressure\_io\_waiting\_seconds\_total{container="io-stress"'
`
```
You will see the metric's value increase as the Pod continuously writes to disk.