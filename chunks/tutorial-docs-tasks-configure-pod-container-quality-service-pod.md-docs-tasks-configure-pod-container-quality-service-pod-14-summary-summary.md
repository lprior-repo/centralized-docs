---
doc_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod
chunk_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod#14-summary
chunk_level: summary
chunk_type: prose
heading: Create a Pod that gets assigned a QoS class of Guaranteed
token_count: 115
summary: Create the Pod: ``` `kubectl apply -f https://k8s.io/examples/pods/qos/qos-pod.yaml --namespace=qos-example ` ``` View detailed information about the Pod: ``` `kubectl get pod qos-demo...
---

Create the Pod:
```
`kubectl apply -f https://k8s.io/examples/pods/qos/qos-pod.yaml --namespace=qos-example
`
```
View detailed information about the Pod:
```
`kubectl get pod qos-demo --namespace=qos-example --output=yaml
`
```
The output shows that Kubernetes gave the Pod a QoS class of `Guaranteed`. The output also
verifies that the Pod Container has a memory request that matches its memory limit, and it has
a CPU request that matches its CPU limit.