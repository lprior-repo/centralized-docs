---
doc_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics
chunk_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics#25-summary
chunk_level: summary
chunk_type: prose
heading: Example Scenarios
token_count: 117
summary: ``` `apiVersion: v1 kind: Pod metadata: name: io-pressure-pod spec: restartPolicy: Never containers: - name: io-stress image: registry.k8s.io/e2e-test-images/agnhost:2.47 command: [\"/bin/sh\", \"-c\"]...
---

```
`apiVersion: v1
kind: Pod
metadata:
name: io-pressure-pod
spec:
restartPolicy: Never
containers:
- name: io-stress
image: registry.k8s.io/e2e-test-images/agnhost:2.47
command: ["/bin/sh", "-c"]
args:
- "while true; do dd if=/dev/zero of=testfile bs=1M count=128 &amp;&gt;/dev/null; sync; rm testfile &amp;&gt;/dev/null; done"
`
```