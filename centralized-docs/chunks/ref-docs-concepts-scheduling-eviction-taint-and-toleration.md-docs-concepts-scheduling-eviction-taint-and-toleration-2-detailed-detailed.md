---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Concepts
token_count: 590
summary: ## Concepts You add a taint to a node using [kubectl taint](/docs/reference/generated/kubectl/kubectl-commands#taint). For example, ``` `kubectl taint nodes node1 key1=value1:NoSchedule ` ``` places...
---

## Concepts
You add a taint to a node using [kubectl taint](/docs/reference/generated/kubectl/kubectl-commands#taint).
For example,
```
`kubectl taint nodes node1 key1=value1:NoSchedule
`
```
places a taint on node `node1`. The taint has key `key1`, value `value1`, and taint effect `NoSchedule`.
This means that no pod will be able to schedule onto `node1` unless it has a matching toleration.
To remove the taint added by the command above, you can run:
```
`kubectl taint nodes node1 key1=value1:NoSchedule-
`
```
You specify a toleration for a pod in the PodSpec. Both of the following tolerations "match" the
taint created by the `kubectl taint` line above, and thus a pod with either toleration would be able
to schedule onto `node1`:
```
`tolerations:
- key: "key1"
operator: "Equal"
value: "value1"
effect: "NoSchedule"
`
```
```
`tolerations:
- key: "key1"
operator: "Exists"
effect: "NoSchedule"
`
```
The default Kubernetes scheduler takes taints and tolerations into account when
selecting a node to run a particular Pod. However, if you manually specify the
`.spec.nodeName` for a Pod, that action bypasses the scheduler; the Pod is then
bound onto the node where you assigned it, even if there are `NoSchedule`
taints on that node that you selected.
If this happens and the node also has a `NoExecute` taint set, the kubelet will
eject the Pod unless there is an appropriate tolerance set.
Here's an example of a pod that has some tolerations defined:
[`pods/pod-with-toleration.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/pod-with-toleration.yaml)![](/images/copycode.svg "Copy pods/pod-with-toleration.yaml to clipboard")
```
`apiVersion: v1
kind: Pod
metadata:
name: nginx
labels:
env: test
spec:
containers:
- name: nginx
image: nginx
imagePullPolicy: IfNotPresent
tolerations:
- key: "example-key"
operator: "Exists"
effect: "NoSchedule"
`
```
The default value for `operator` is `Equal`.
A toleration "matches" a taint if the keys are the same and the effects are the same, and:
* the `operator` is `Exists` (in which case no `value` should be specified), or
* the `operator` is `Equal` and the values should be equal.