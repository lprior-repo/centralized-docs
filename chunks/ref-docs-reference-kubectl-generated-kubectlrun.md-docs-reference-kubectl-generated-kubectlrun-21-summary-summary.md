---
doc_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun
chunk_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun#21-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 98
summary: || Process the directory used in -f, --filename recursively. Useful when you want to manage related manifests organized within the same directory. | |--restart stringDefault: \"Always\"| || The restart...
---

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