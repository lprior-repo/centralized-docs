---
doc_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod
chunk_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod#25-summary
chunk_level: summary
chunk_type: prose
heading: Create a Pod that gets assigned a QoS class of BestEffort
token_count: 100
summary: View detailed information about the Pod: ``` `kubectl get pod qos-demo-3 --namespace=qos-example --output=yaml ` ``` The output shows that Kubernetes gave the Pod a QoS class of `BestEffort`: ```...
---

View detailed information about the Pod:
```
`kubectl get pod qos-demo-3 --namespace=qos-example --output=yaml
`
```
The output shows that Kubernetes gave the Pod a QoS class of `BestEffort`:
```
`spec:
containers:
...
resources: {}
...
status:
qosClass: BestEffort
`
```
#### Clean up
Delete your Pod:
```
`kubectl delete pod qos-demo-3 --namespace=qos-example
`
```