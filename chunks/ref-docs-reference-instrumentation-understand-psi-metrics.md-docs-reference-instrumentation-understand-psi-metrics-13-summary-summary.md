---
doc_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics
chunk_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics#13-summary
chunk_level: summary
chunk_type: prose
heading: Example Scenarios
token_count: 111
summary: ``` `apiVersion: v1 kind: Pod metadata: name: cpu-pressure-pod spec: restartPolicy: Never containers: - name: cpu-stress image: registry.k8s.io/e2e-test-images/agnhost:2.47 args: - \"stress\" -...
---

```
`apiVersion: v1
kind: Pod
metadata:
name: cpu-pressure-pod
spec:
restartPolicy: Never
containers:
- name: cpu-stress
image: registry.k8s.io/e2e-test-images/agnhost:2.47
args:
- "stress"
- "--cpus"
- "1"
resources:
limits:
cpu: "500m"
requests:
cpu: "500m"
`
```
Apply it to your cluster: `kubectl apply -f cpu-pressure-pod.yaml`