---
doc_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun
chunk_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun#4-standard
chunk_level: standard
chunk_type: table
heading: Options
token_count: 483
summary: --force is true (force deletion). | |-h, --help| || help for run | |--image string| || The image for the container to run. | |--image-pull-policy string| || The image pull policy for the container....
---

--force is true (force deletion).
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
Process a kustomization directory. This flag can't be used together with -f or -R.
|
|-l, --labels string|
||
Comma separated labels to apply to the pod. Will override previous values.
|
|--leave-stdin-open|
||
If the pod is started in interactive mode or with stdin, leave stdin open after the first attach completes. By default, stdin will be closed after the first attach completes.
|
|-o, --output string|
||
Output format. One of: (json, yaml, kyaml, name, go-template, go-template-file, template, templatefile, jsonpath, jsonpath-as-json, jsonpath-file).
|
|--override-type stringDefault: "merge"|
||
The method used to override the generated object: json, merge, or strategic.
|
|--overrides string|
||
An inline JSON override for the generated object. If this is non-empty, it is used to override the generated object. Requires that the object supply a valid apiVersion field.
|
|--pod-running-timeout durationDefault: 1m0s|
||
The length of time (like 5s, 2m, or 3h, higher than zero) to wait until at least one pod is running
|
|--port string|
||
The port that this container exposes.
|
|--privileged|
||
If true, run the container in privileged mode.
|
|-q, --quiet|
||
If true, suppress prompt messages.
|
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