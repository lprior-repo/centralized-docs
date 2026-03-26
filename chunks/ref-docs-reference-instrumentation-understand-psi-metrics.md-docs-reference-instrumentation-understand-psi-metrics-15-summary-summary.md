---
doc_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics
chunk_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics#15-summary
chunk_level: summary
chunk_type: prose
heading: Example Scenarios
token_count: 79
summary: You will see the `some` PSI metrics for CPU increase in the summary API output. The `avg10` value for `some` pressure should rise above zero, indicating that tasks are spending time stalled on the...
---

You will see the `some` PSI metrics for CPU increase in the summary API output. The `avg10` value for `some` pressure should rise above zero, indicating that tasks are spending time stalled on the CPU.
**Using the Prometheus metrics endpoint:**
Query the `/metrics/cadvisor` endpoint to see the `container\_pressure\_cpu\_waiting\_seconds\_total` metric.