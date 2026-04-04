---
doc_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness
chunk_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness#2-detailed
chunk_level: detailed
chunk_type: code
heading: What's next
token_count: 983
summary: ## Usage example To mark a Pod not-ready for scheduling, you can create it with one or more scheduling gates like this: [`pods/pod-with-scheduling-gates.yaml`...
---

## Usage example
To mark a Pod not-ready for scheduling, you can create it with one or more scheduling gates like this:
[`pods/pod-with-scheduling-gates.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/pod-with-scheduling-gates.yaml)![](/images/copycode.svg "Copy pods/pod-with-scheduling-gates.yaml to clipboard")
```
`apiVersion: v1
kind: Pod
metadata:
name: test-pod
spec:
schedulingGates:
- name: example.com/foo
- name: example.com/bar
containers:
- name: pause
image: registry.k8s.io/pause:3.6
`
```
After the Pod's creation, you can check its state using:
```
`kubectl get pod test-pod
`
```
The output reveals it's in `SchedulingGated` state:
```
`NAME READY STATUS RESTARTS AGE
test-pod 0/1 SchedulingGated 0 7s
`
```
You can also check its `schedulingGates` field by running:
```
`kubectl get pod test-pod -o jsonpath='{.spec.schedulingGates}'
`
```
The output is:
```
`[{"name":"example.com/foo"},{"name":"example.com/bar"}]
`
```
To inform scheduler this Pod is ready for scheduling, you can remove its `schedulingGates` entirely
by reapplying a modified manifest:
[`pods/pod-without-scheduling-gates.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/pod-without-scheduling-gates.yaml)![](/images/copycode.svg "Copy pods/pod-without-scheduling-gates.yaml to clipboard")
```
`apiVersion: v1
kind: Pod
metadata:
name: test-pod
spec:
containers:
- name: pause
image: registry.k8s.io/pause:3.6
`
```
You can check if the `schedulingGates` is cleared by running:
```
`kubectl get pod test-pod -o jsonpath='{.spec.schedulingGates}'
`
```
The output is expected to be empty. And you can check its latest status by running:
```
`kubectl get pod test-pod -o wide
`
```
Given the test-pod doesn't request any CPU/memory resources, it's expected that this Pod's state get
transited from previous `SchedulingGated` to `Running`:
```
`NAME READY STATUS RESTARTS AGE IP NODE
test-pod 1/1 Running 0 15s 10.0.0.4 node-2
`
```
## Observability
The metric `scheduler\_pending\_pods` comes with a new label `"gated"` to distinguish whether a Pod
has been tried scheduling but claimed as unschedulable, or explicitly marked as not ready for
scheduling. You can use `scheduler\_pending\_pods{queue="gated"}` to check the metric result.
## Mutable Pod scheduling directives
You can mutate scheduling directives of Pods while they have scheduling gates, with certain constraints.
At a high level, you can only tighten the scheduling directives of a Pod. In other words, the updated
directives would cause the Pods to only be able to be scheduled on a subset of the nodes that it would
previously match. More concretely, the rules for updating a Pod's scheduling directives are as follows:
1. For `.spec.nodeSelector`, only additions are allowed. If absent, it will be allowed to be set.
2. For `spec.affinity.nodeAffinity`, if nil, then setting anything is allowed.
3. If `NodeSelectorTerms` was empty, it will be allowed to be set.
If not empty, then only additions of `NodeSelectorRequirements` to `matchExpressions`
or `fieldExpressions` are allowed, and no changes to existing `matchExpressions`
and `fieldExpressions` will be allowed. This is because the terms in
`.requiredDuringSchedulingIgnoredDuringExecution.NodeSelectorTerms`, are ORed
while the expressions in `nodeSelectorTerms[].matchExpressions` and
`nodeSelectorTerms[].fieldExpressions` are ANDed.
4. For `.preferredDuringSchedulingIgnoredDuringExecution`, all updates are allowed.
This is because preferred terms are not authoritative, and so policy controllers
don't validate those terms.
## What's next
* Read the [PodSchedulingReadiness KEP](https://github.com/kubernetes/enhancements/blob/master/keps/sig-scheduling/3521-pod-scheduling-readiness) for more details