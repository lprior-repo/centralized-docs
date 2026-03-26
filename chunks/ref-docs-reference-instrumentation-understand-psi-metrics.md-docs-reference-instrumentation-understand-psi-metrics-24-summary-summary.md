---
doc_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics
chunk_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics#24-summary
chunk_level: summary
chunk_type: prose
heading: Example Scenarios
token_count: 48
summary: ### Generating I/O Pressure This Pod generates I/O pressure by repeatedly writing a file to disk and using `sync` to flush the data from memory, which creates I/O stalls. Create a file named...
---

### Generating I/O Pressure
This Pod generates I/O pressure by repeatedly writing a file to disk and using `sync` to flush the data from memory, which creates I/O stalls.
Create a file named `io-pressure-pod.yaml`: