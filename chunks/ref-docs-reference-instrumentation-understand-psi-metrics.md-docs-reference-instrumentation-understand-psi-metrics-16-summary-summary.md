---
doc_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics
chunk_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics#16-summary
chunk_level: summary
chunk_type: prose
heading: Example Scenarios
token_count: 118
summary: ``` `# Replace &lt;node-name&gt; with the name of the node where the pod is running kubectl get --raw \"/api/v1/nodes/&lt;node-name&gt;/proxy/metrics/cadvisor\" | \\ grep...
---

```
`# Replace &lt;node-name&gt; with the name of the node where the pod is running
kubectl get --raw "/api/v1/nodes/&lt;node-name&gt;/proxy/metrics/cadvisor" | \\
grep 'container\_pressure\_cpu\_waiting\_seconds\_total{container="cpu-stress"'
`
```
The output should show an increasing value, indicating that the container is spending time stalled waiting for CPU resources.
#### Cleanup
Clean up the Pod when you are finished:
```
`kubectl delete pod cpu-pressure-pod
`
```