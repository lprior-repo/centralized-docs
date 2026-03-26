---
doc_id: ref/docs-reference-config-api-kuberc-v1alpha1.md/docs-reference-config-api-kuberc-v1alpha1
chunk_id: ref/docs-reference-config-api-kuberc-v1alpha1.md/docs-reference-config-api-kuberc-v1alpha1#6-summary
chunk_level: summary
chunk_type: prose
heading: Resource Types
token_count: 77
summary: * name: getn command: get flags: * name: output default: wide prependArgs: * node \"kubectl getn control-plane-1\" expands to \"kubectl get node control-plane-1 --output=wide\" \"kubectl getn...
---

* name: getn
command: get
flags:
* name: output
default: wide
prependArgs:
* node
"kubectl getn control-plane-1" expands to "kubectl get node control-plane-1 --output=wide"
"kubectl getn control-plane-1 --output=json" expands to "kubectl get node --output=json control-plane-1"|