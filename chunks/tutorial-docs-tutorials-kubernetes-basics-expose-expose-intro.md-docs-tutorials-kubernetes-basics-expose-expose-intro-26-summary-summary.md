---
doc_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro#26-summary
chunk_level: summary
chunk_type: prose
heading: Services and Labels
token_count: 118
summary: To apply a new label we use the label subcommand followed by the object type, object name and the new label: ``` `kubectl label pods \"$POD\_NAME\" version=v1 ` ``` This will apply a new label to our...
---

To apply a new label we use the label subcommand followed by the object type,
object name and the new label:
```
`kubectl label pods "$POD\_NAME" version=v1
`
```
This will apply a new label to our Pod (we pinned the application version to the Pod),
and we can check it with the `describe pod` command:
```
`kubectl describe pods "$POD\_NAME"
`
```
We see here that the label is attached now to our Pod. And we can query now the
list of pods using the new label: