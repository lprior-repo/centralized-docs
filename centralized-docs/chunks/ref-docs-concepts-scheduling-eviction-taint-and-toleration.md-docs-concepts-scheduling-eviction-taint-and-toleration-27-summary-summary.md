---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#27-summary
chunk_level: summary
chunk_type: prose
heading: Numeric comparison operators
token_count: 99
summary: ``` `kubectl taint nodes node1 servicelevel.organization.example/agreed-service-level=950:NoSchedule ` ``` A pod can tolerate nodes with SLA greater than 900: [`pods/pod-with-numeric-toleration.yaml`...
---

```
`kubectl taint nodes node1 servicelevel.organization.example/agreed-service-level=950:NoSchedule
`
```
A pod can tolerate nodes with SLA greater than 900:
[`pods/pod-with-numeric-toleration.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/pod-with-numeric-toleration.yaml)![](/images/copycode.svg "Copy pods/pod-with-numeric-toleration.yaml to clipboard")