---
doc_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod
chunk_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod#30-summary
chunk_level: summary
chunk_type: prose
heading: Retrieve the QoS class for a Pod
token_count: 68
summary: ## Retrieve the QoS class for a Pod Rather than see all the fields, you can view just the field you need: ``` `kubectl --namespace=qos-example get pod qos-demo-4 -o jsonpath='{...
---

## Retrieve the QoS class for a Pod
Rather than see all the fields, you can view just the field you need:
```
`kubectl --namespace=qos-example get pod qos-demo-4 -o jsonpath='{ .status.qosClass}{"\\n"}'
`
```
```
`Burstable
`
```