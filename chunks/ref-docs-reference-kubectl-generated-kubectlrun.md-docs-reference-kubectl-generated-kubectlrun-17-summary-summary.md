---
doc_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun
chunk_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun#17-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 127
summary: | |--grace-period intDefault: -1| || Period of time in seconds given to the resource to terminate gracefully. Ignored if negative. Set to 1 for immediate shutdown. Can only be set to 0 when --force...
---

|
|--grace-period intDefault: -1|
||
Period of time in seconds given to the resource to terminate gracefully. Ignored if negative. Set to 1 for immediate shutdown. Can only be set to 0 when --force is true (force deletion).
|
|-h, --help|
||
help for run
|
|--image string|
||
The image for the container to run.
|
|--image-pull-policy string|
||
The image pull policy for the container. If left empty, this value will not be specified by the client and defaulted by the server.
|
|-k, --kustomize string|
||