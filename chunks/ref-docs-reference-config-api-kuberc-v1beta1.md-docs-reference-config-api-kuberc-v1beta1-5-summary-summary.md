---
doc_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1
chunk_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1#5-summary
chunk_level: summary
chunk_type: prose
heading: Resource Types
token_count: 91
summary: * name: runx command: run options: * name: image default: nginx appendArgs: * custom-arg1 For example, if user invokes \"kubectl runx test-pod\" command, this will be expanded to \"kubectl run...
---

* name: runx
command: run
options:
* name: image
default: nginx
appendArgs:
* custom-arg1
For example, if user invokes "kubectl runx test-pod" command,
this will be expanded to "kubectl run --image=nginx test-pod -- custom-arg1"
* name: getn
command: get
options:
* name: output
default: wide
prependArgs: