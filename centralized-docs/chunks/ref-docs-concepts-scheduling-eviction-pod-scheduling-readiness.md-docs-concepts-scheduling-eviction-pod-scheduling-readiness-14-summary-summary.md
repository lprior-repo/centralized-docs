---
doc_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness
chunk_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness#14-summary
chunk_level: summary
chunk_type: prose
heading: Usage example
token_count: 115
summary: The output is: ``` `[{\"name\":\"example.com/foo\"},{\"name\":\"example.com/bar\"}] ` ``` To inform scheduler this Pod is ready for scheduling, you can remove its `schedulingGates` entirely by reapplying a...
---

The output is:
```
`[{"name":"example.com/foo"},{"name":"example.com/bar"}]
`
```
To inform scheduler this Pod is ready for scheduling, you can remove its `schedulingGates` entirely
by reapplying a modified manifest:
[`pods/pod-without-scheduling-gates.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/pod-without-scheduling-gates.yaml)![](/images/copycode.svg "Copy pods/pod-without-scheduling-gates.yaml to clipboard")