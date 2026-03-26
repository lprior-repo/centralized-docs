---
doc_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics
chunk_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics#2-detailed
chunk_level: detailed
chunk_type: code
heading: Example Scenarios
token_count: 996
summary: ## Example Scenarios You can use a simple Pod with a stress-testing tool to generate resource pressure and observe the PSI metrics. The following examples use the `agnhost` container image, which...
---

## Example Scenarios
You can use a simple Pod with a stress-testing tool to generate resource pressure and observe the PSI metrics. The following examples use the `agnhost` container image, which includes the `stress` tool.
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
### Generating Memory Pressure
This example creates a Pod that continuously writes to files in the container's writable layer, causing the kernel's page cache to grow and forcing memory reclamation, which generates pressure.
Create a file named `memory-pressure-pod.yaml`:
```
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
memory: "200M"
requests:
memory: "200M"
`
```
Apply it to your cluster: `kubectl apply -f memory-pressure-pod.yaml`
#### Observing Memory Pressure
**Using the Summary API:**
In the summary output, you will observe an increase in the `full` PSI metrics for memory, indicating that the system is under significant memory pressure.
```
`# Replace &lt;node-name&gt; with the name of a node in your cluster
kubectl get --raw "/api/v1/nodes/&lt;node-name&gt;/proxy/stats/summary" | jq '.pods[] | select(.podRef.name | contains("memory-pressure-pod"))'
`
```
**Using the Prometheus metrics endpoint:**
Query the `/metrics/cadvisor` endpoint to see the `container\_pressure\_memory\_waiting\_seconds\_total` metric.
```
`# Replace &lt;node-name&gt; with the name of the node where the pod is running
kubectl get --raw "/api/v1/nodes/&lt;node-name&gt;/proxy/metrics/cadvisor" | \\
grep 'container\_pressure\_memory\_waiting\_seconds\_total{container="memory-stress"'
`
```
In the output, you will observe an increasing value for the metric, indicating that the system is under significant memory pressure.
#### Cleanup
Clean up the Pod when you are finished:
```
`kubectl delete pod memory-pressure-pod
`
```