---
doc_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod
chunk_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod#12-summary
chunk_level: summary
chunk_type: prose
heading: Create a Pod that gets assigned a QoS class of Guaranteed
token_count: 110
summary: * For every Container in the Pod, the CPU limit must equal the CPU request. These restrictions apply to init containers and app containers equally. [Ephemeral...
---

* For every Container in the Pod, the CPU limit must equal the CPU request.
These restrictions apply to init containers and app containers equally.
[Ephemeral containers](/docs/concepts/workloads/pods/ephemeral-containers/)
cannot define resources so these restrictions do not apply.
Here is a manifest for a Pod that has one Container. The Container has a memory limit and a
memory request, both equal to 200 MiB. The Container has a CPU limit and a CPU request, both equal to 700 milliCPU: