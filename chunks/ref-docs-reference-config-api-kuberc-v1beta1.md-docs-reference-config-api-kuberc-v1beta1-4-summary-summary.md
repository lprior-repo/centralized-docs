---
doc_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1
chunk_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1#4-summary
chunk_level: summary
chunk_type: prose
heading: Resource Types
token_count: 115
summary: kubectl [ALIAS NAME] [USER\_OPTIONS] [USER\_EXPLICIT\_ARGS] expands to kubectl [COMMAND] # built-in command alias points to [KUBERC\_PREPEND\_ARGS] [USER\_OPTIONS] [KUBERC\_OPTIONS] # rest of the...
---

kubectl [ALIAS NAME] [USER\_OPTIONS] [USER\_EXPLICIT\_ARGS] expands to
kubectl [COMMAND] # built-in command alias points to
[KUBERC\_PREPEND\_ARGS]
[USER\_OPTIONS]
[KUBERC\_OPTIONS] # rest of the options that are not passed by user in [USER\_OPTIONS]
[USER\_EXPLICIT\_ARGS]
[KUBERC\_APPEND\_ARGS]
e.g.
* name: runx
command: run
options:
* name: image
default: nginx
appendArgs: