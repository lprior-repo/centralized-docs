---
doc_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics
chunk_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics#17-summary
chunk_level: summary
chunk_type: prose
heading: Example Scenarios
token_count: 51
summary: ### Generating Memory Pressure This example creates a Pod that continuously writes to files in the container's writable layer, causing the kernel's page cache to grow and forcing memory reclamation,...
---

### Generating Memory Pressure
This example creates a Pod that continuously writes to files in the container's writable layer, causing the kernel's page cache to grow and forcing memory reclamation, which generates pressure.
Create a file named `memory-pressure-pod.yaml`: