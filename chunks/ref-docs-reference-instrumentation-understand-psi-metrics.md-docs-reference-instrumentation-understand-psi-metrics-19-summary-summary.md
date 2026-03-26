---
doc_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics
chunk_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics#19-summary
chunk_level: summary
chunk_type: prose
heading: Example Scenarios
token_count: 125
summary: `apiVersion: v1 kind: Pod metadata: name: memory-pressure-pod spec: restartPolicy: Never containers: - name: memory-stress image: registry.k8s.io/e2e-test-images/agnhost:2.47 command: [\"/bin/sh\",...
---

`apiVersion: v1
kind: Pod
metadata:
name: memory-pressure-pod
spec:
restartPolicy: Never
containers:
- name: memory-stress
image: registry.k8s.io/e2e-test-images/agnhost:2.47
command: ["/bin/sh", "-c"]
args:
- "i=0; while true; do dd if=/dev/zero of=testfile.$i bs=1M count=50 &amp;&gt;/dev/null; i=$(((i+1)%5)); sleep 0.1; done"
resources:
limits: