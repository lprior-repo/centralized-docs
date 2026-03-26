---
doc_id: ref/docs-reference-config-api-kuberc-v1alpha1.md/docs-reference-config-api-kuberc-v1alpha1
chunk_id: ref/docs-reference-config-api-kuberc-v1alpha1.md/docs-reference-config-api-kuberc-v1alpha1#4-summary
chunk_level: summary
chunk_type: prose
heading: Resource Types
token_count: 115
summary: kubectl [ALIAS NAME] [USER\_FLAGS] [USER\_EXPLICIT\_ARGS] expands to kubectl [COMMAND] # built-in command alias points to [KUBERC\_PREPEND\_ARGS] [USER\_FLAGS] [KUBERC\_FLAGS] # rest of the flags...
---

kubectl [ALIAS NAME] [USER\_FLAGS] [USER\_EXPLICIT\_ARGS] expands to
kubectl [COMMAND] # built-in command alias points to
[KUBERC\_PREPEND\_ARGS]
[USER\_FLAGS]
[KUBERC\_FLAGS] # rest of the flags that are not passed by user in [USER\_FLAGS]
[USER\_EXPLICIT\_ARGS]
[KUBERC\_APPEND\_ARGS]
e.g.
* name: runx
command: run
flags:
* name: image
default: nginx
appendArgs: