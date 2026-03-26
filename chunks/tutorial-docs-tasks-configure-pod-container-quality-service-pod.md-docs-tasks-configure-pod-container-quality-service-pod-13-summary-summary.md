---
doc_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod
chunk_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod#13-summary
chunk_level: summary
chunk_type: prose
heading: Create a Pod that gets assigned a QoS class of Guaranteed
token_count: 127
summary: [`pods/qos/qos-pod.yaml` ](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/qos/qos-pod.yaml)![](/images/copycode.svg \"Copy pods/qos/qos-pod.yaml to clipboard\") ```...
---

[`pods/qos/qos-pod.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/qos/qos-pod.yaml)![](/images/copycode.svg "Copy pods/qos/qos-pod.yaml to clipboard")
```
`apiVersion: v1
kind: Pod
metadata:
name: qos-demo
namespace: qos-example
spec:
containers:
- name: qos-demo-ctr
image: nginx
resources:
limits:
memory: "200Mi"
cpu: "700m"
requests:
memory: "200Mi"
cpu: "700m"
`
```