---
doc_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics
chunk_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics#3-detailed
chunk_level: detailed
chunk_type: table
heading: Related Pages
token_count: 789
summary: ### Generating I/O Pressure This Pod generates I/O pressure by repeatedly writing a file to disk and using `sync` to flush the data from memory, which creates I/O stalls. Create a file named...
---

### Generating I/O Pressure
This Pod generates I/O pressure by repeatedly writing a file to disk and using `sync` to flush the data from memory, which creates I/O stalls.
Create a file named `io-pressure-pod.yaml`:
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
Apply this to your cluster: `kubectl apply -f io-pressure-pod.yaml`
#### Observing I/O Pressure
**Using the Summary API:**
You will see the `some` PSI metrics for I/O increase as the Pod continuously writes to disk.
```
`# Replace &lt;node-name&gt; with the name of a node in your cluster
kubectl get --raw "/api/v1/nodes/&lt;node-name&gt;/proxy/stats/summary" | jq '.pods[] | select(.podRef.name | contains("io-pressure-pod"))'
`
```
**Using the Prometheus metrics endpoint:**
Query the `/metrics/cadvisor` endpoint to see the `container\_pressure\_io\_waiting\_seconds\_total` metric.
```
`# Replace &lt;node-name&gt; with the name of the node where the pod is running
kubectl get --raw "/api/v1/nodes/&lt;node-name&gt;/proxy/metrics/cadvisor" | \\
grep 'container\_pressure\_io\_waiting\_seconds\_total{container="io-stress"'
`
```
You will see the metric's value increase as the Pod continuously writes to disk.
#### Cleanup
Clean up the Pod when you are finished:
```
`kubectl delete pod io-pressure-pod
`
```
## What's next
The task pages for [Troubleshooting Clusters](/docs/tasks/debug/debug-cluster/) discuss
how to use a metrics pipeline that rely on these data.
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified November 30, 2025 at 3:34 PM PST: [Correct moving average duration in PSI metrics description (997b8d14fd)](https://github.com/kubernetes/website/commit/997b8d14fd5f98e6c106bda113c02758c246dbd1)
## Related Pages

- [Debug Pods](docs-tasks-debug-debug-application-debug-pods.md)
- [Kubernetes Component SLI Metrics](docs-reference-instrumentation-slis.md)
- [Switching from Polling to CRI Event-based Updates to Container Status](docs-tasks-administer-cluster-switch-to-evented-pleg.md)
- [Adopting Sidecar Containers](docs-tutorials-configuration-pod-sidecar-containers.md)
- [Adding entries to Pod /etc/hosts with HostAliases](docs-tasks-network-customize-hosts-file-for-pods.md)