---
doc_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod
chunk_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod#20-summary
chunk_level: summary
chunk_type: prose
heading: Create a Pod that gets assigned a QoS class of Burstable
token_count: 105
summary: View detailed information about the Pod: ``` `kubectl get pod qos-demo-2 --namespace=qos-example --output=yaml ` ``` The output shows that Kubernetes gave the Pod a QoS class of `Burstable`: ```...
---

View detailed information about the Pod:
```
`kubectl get pod qos-demo-2 --namespace=qos-example --output=yaml
`
```
The output shows that Kubernetes gave the Pod a QoS class of `Burstable`:
```
`spec:
containers:
- image: nginx
imagePullPolicy: Always
name: qos-demo-2-ctr
resources:
limits:
memory: 200Mi
requests:
memory: 100Mi
...
status:
qosClass: Burstable
`
```