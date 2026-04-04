---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#8-standard
chunk_level: standard
chunk_type: prose
heading: Numeric comparison operators
token_count: 415
summary: #### Note: When you create a Pod that uses `Gt` or `Lt` tolerations operators, the API server validates that the toleration values are valid integers. Taint values on nodes are not validated at node...
---

#### Note:
When you create a Pod that uses `Gt` or `Lt` tolerations operators, the API server validates that
the toleration values are valid integers. Taint values on nodes are not validated at node
registration time. If a node has a non-numeric taint value (for example,
`servicelevel.organization.example/agreed-service-level=high:NoSchedule`),
pods with numeric comparison operators will not match that taint and cannot schedule on that node.
For example, if nodes are tainted with a value representing a service level agreement (SLA):
```
`kubectl taint nodes node1 servicelevel.organization.example/agreed-service-level=950:NoSchedule
`
```
A pod can tolerate nodes with SLA greater than 900:
[`pods/pod-with-numeric-toleration.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/pod-with-numeric-toleration.yaml)![](/images/copycode.svg "Copy pods/pod-with-numeric-toleration.yaml to clipboard")
```
`apiVersion: v1
kind: Pod
metadata:
name: nginx-numeric-toleration
labels:
env: test
spec:
containers:
- name: nginx
image: nginx
imagePullPolicy: IfNotPresent
tolerations:
- key: "servicelevel.organization.example/agreed-service-level"
operator: "Gt"
value: "900"
effect: "NoSchedule"
`
```
This toleration matches the taint on `node1` because `950 &gt; 900` (the taint value
is greater than the toleration value for the `Gt` operator).
Similarly, you can use the `Lt` operator to match taints where the taint value is
less than the toleration value:
```
`tolerations:
- key: "servicelevel.organization.example/agreed-service-level"
operator: "Lt"
value: "1000"
effect: "NoSchedule"
`
```