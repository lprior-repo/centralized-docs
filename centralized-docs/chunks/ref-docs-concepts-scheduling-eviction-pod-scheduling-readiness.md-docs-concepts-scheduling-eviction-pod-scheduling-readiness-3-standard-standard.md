---
doc_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness
chunk_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness#3-standard
chunk_level: standard
chunk_type: code
heading: Usage example
token_count: 494
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