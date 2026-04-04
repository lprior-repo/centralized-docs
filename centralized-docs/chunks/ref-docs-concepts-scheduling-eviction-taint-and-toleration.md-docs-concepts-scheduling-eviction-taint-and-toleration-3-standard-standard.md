---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#3-standard
chunk_level: standard
chunk_type: prose
heading: Concepts
token_count: 145
summary: ``` `apiVersion: v1 kind: Pod metadata: name: nginx labels: env: test spec: containers: - name: nginx image: nginx imagePullPolicy: IfNotPresent tolerations: - key: \"example-key\" operator: \"Exists\"...
---

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