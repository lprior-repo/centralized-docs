---
doc_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun
chunk_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun#4-detailed
chunk_level: detailed
chunk_type: table
heading: Options
token_count: 318
summary: |-R, --recursive| || Process the directory used in -f, --filename recursively. Useful when you want to manage related manifests organized within the same directory. | |--restart stringDefault:...
---

|-R, --recursive|
||
Process the directory used in -f, --filename recursively. Useful when you want to manage related manifests organized within the same directory.
|
|--restart stringDefault: "Always"|
||
The restart policy for this Pod. Legal values [Always, OnFailure, Never].
|
|--rm|
||
If true, delete the pod after it exits. Only valid when attaching to the container, e.g. with '--attach' or with '-i/--stdin'.
|
|--save-config|
||
If true, the configuration of current object will be saved in its annotation. Otherwise, the annotation will be unchanged. This flag is useful when you want to perform kubectl apply on this object in the future.
|
|--show-managed-fields|
||
If true, keep the managedFields when printing objects in JSON or YAML format.
|
|-i, --stdin|
||
Keep stdin open on the container in the pod, even if nothing is attached.
|
|--template string|
||
Template string or path to template file to use when -o=go-template, -o=go-template-file. The template format is golang templates [http://golang.org/pkg/text/template/#pkg-overview].
|
|--timeout duration|
||
The length of time to wait before giving up on a delete, zero means determine a timeout from the size of the object
|
|-t, --tty|
||
Allocate a TTY for the container in the pod.
|
|--wait|
||
If true, wait for resources to be gone before returning. This waits for finalizers.
|