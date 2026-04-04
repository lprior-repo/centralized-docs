---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#50-summary
chunk_level: summary
chunk_type: table
heading: Referring to Kubernetes API resources
token_count: 115
summary: ### Don't include the command prompt Do and Don't - Don't include the command prompt|Do|Don't| |`kubectl get pods`|`$ kubectl get pods`| ### Separate commands from output Verify that the pod is...
---

### Don't include the command prompt
Do and Don't - Don't include the command prompt|Do|Don't|
|`kubectl get pods`|`$ kubectl get pods`|
### Separate commands from output
Verify that the pod is running on your chosen node:
```
`kubectl get pods --output=wide
`
```
The output is similar to this:
```
`NAME READY STATUS RESTARTS AGE IP NODE
nginx 1/1 Running 0 13s 10.200.0.4 worker0
`
```