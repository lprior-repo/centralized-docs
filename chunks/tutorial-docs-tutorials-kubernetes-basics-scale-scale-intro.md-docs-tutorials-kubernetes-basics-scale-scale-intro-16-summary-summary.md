---
doc_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro#16-summary
chunk_level: summary
chunk_type: prose
heading: Scaling overview
token_count: 59
summary: ``` `kubectl get rs ` ``` Notice that the name of the ReplicaSet is always formatted as [DEPLOYMENT-NAME]-[RANDOM-STRING]. The random string is randomly generated and uses the pod-template-hash as a...
---

```
`kubectl get rs
`
```
Notice that the name of the ReplicaSet is always formatted as
[DEPLOYMENT-NAME]-[RANDOM-STRING].
The random string is randomly generated and uses the pod-template-hash as a seed.
Two important columns of this output are: