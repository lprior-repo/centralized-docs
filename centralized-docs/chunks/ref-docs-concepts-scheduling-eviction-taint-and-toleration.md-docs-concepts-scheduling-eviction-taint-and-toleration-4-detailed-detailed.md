---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#4-detailed
chunk_level: detailed
chunk_type: prose
heading: Numeric comparison operators
token_count: 811
summary: ## Numeric comparison operators FEATURE STATE: `Kubernetes v1.35 [alpha]`(disabled by default) In addition to `Equal` and `Exists`, you can use numeric comparison operators (`Gt` and `Lt`) to match...
---

## Numeric comparison operators
FEATURE STATE:
`Kubernetes v1.35 [alpha]`(disabled by default)
In addition to `Equal` and `Exists`, you can use numeric comparison operators
(`Gt` and `Lt`) to match taints with integer values. This is useful for threshold-based
scheduling, such as matching nodes by reliability level or SLA tier.
* `Gt` matches when the taint value is greater than the toleration value.
* `Lt` matches when the taint value is less than the toleration value.
For numeric operators, both the toleration and taint values must be valid integers.
If either value cannot be parsed as an integer, the toleration does not match.
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
#### Note:
When using numeric comparison operators:
* Both the toleration and taint values must be valid signed 64-bit integers
(zero leading numbers (e.g., "0550") are not allowed).
* If a value cannot be parsed as an integer, the toleration does not match.
* Numeric operators work with all taint effects: `NoSchedule`, `PreferNoSchedule`, and `NoExecute`.
* For `PreferNoSchedule` with numeric operators: if a pod's toleration doesn't satisfy the numeric comparison
(e.g., taint value &lt; toleration value when using `Gt`), the scheduler gives the node a lower priority
but may still schedule there if no better options exist.
#### Warning:
Before disabling the `TaintTolerationComparisonOperators` feature gate:
* You should identify all workloads using the `Gt` or `Lt` operators to avoid controller hot-loops.
* Update all workload controller templates to use `Equal` or `Exists` operators instead
* Delete any pending pods that use `Gt` or `Lt` operators
* Monitor the `apiserver\_request\_total` metric for spikes in validation errors