---
doc_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics
chunk_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics#4-standard
chunk_level: standard
chunk_type: table
heading: Example Scenarios
token_count: 470
summary: ### Generating CPU Pressure Create a Pod that generates CPU pressure using the `stress` utility. This workload will put a heavy load on one CPU core. Create a file named `cpu-pressure-pod.yaml`: ```...
---

### Generating CPU Pressure
Create a Pod that generates CPU pressure using the `stress` utility. This workload will put a heavy load on one CPU core.
Create a file named `cpu-pressure-pod.yaml`:
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
#### Observing CPU Pressure
After the Pod is running, you can observe the CPU pressure through either the Summary API or the Prometheus metrics endpoint.
**Using the Summary API:**
Watch the summary stats for your node. In a separate terminal, run:
```
`# Replace &lt;node-name&gt; with the name of a node in your cluster
kubectl get --raw "/api/v1/nodes/&lt;node-name&gt;/proxy/stats/summary" | jq '.pods[] | select(.podRef.name | contains("cpu-pressure-pod"))'
`
```
You will see the `some` PSI metrics for CPU increase in the summary API output. The `avg10` value for `some` pressure should rise above zero, indicating that tasks are spending time stalled on the CPU.
**Using the Prometheus metrics endpoint:**
Query the `/metrics/cadvisor` endpoint to see the `container\_pressure\_cpu\_waiting\_seconds\_total` metric.
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