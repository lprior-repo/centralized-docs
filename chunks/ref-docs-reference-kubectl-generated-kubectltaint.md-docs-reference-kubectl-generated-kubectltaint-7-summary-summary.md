---
doc_id: ref/docs-reference-kubectl-generated-kubectltaint.md/docs-reference-kubectl-generated-kubectltaint
chunk_id: ref/docs-reference-kubectl-generated-kubectltaint.md/docs-reference-kubectl-generated-kubectltaint#7-summary
chunk_level: summary
chunk_type: prose
heading: Synopsis
token_count: 67
summary: * The effect must be NoSchedule, PreferNoSchedule or NoExecute. * Currently taint can only apply to node. ``` `kubectl taint NODE NAME KEY\_1=VAL\_1:TAINT\_EFFECT\_1 ......
---

* The effect must be NoSchedule, PreferNoSchedule or NoExecute.
* Currently taint can only apply to node.
```
`kubectl taint NODE NAME KEY\_1=VAL\_1:TAINT\_EFFECT\_1 ... KEY\_N=VAL\_N:TAINT\_EFFECT\_N
`
```