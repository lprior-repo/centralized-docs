---
doc_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun
chunk_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun#3-detailed
chunk_level: detailed
chunk_type: table
heading: Options
token_count: 1014
summary: |--allow-missing-template-keysDefault: true| || If true, ignore any errors in templates when a field or map key is missing in the template. Only applies to golang and jsonpath output formats. |...
---

|--allow-missing-template-keysDefault: true|
||
If true, ignore any errors in templates when a field or map key is missing in the template. Only applies to golang and jsonpath output formats.
|
|--annotations strings|
||
Annotations to apply to the pod.
|
|--attach|
||
If true, wait for the Pod to start running, and then attach to the Pod as if 'kubectl attach ...' were called. Default false, unless '-i/--stdin' is set, in which case the default is true. With '--restart=Never' the exit code of the container process is returned.
|
|--cascade string[="background"]Default: "background"|
||
Must be "background", "orphan", or "foreground". Selects the deletion cascading strategy for the dependents (e.g. Pods created by a ReplicationController). Defaults to background.
|
|--command|
||
If true and extra arguments are present, use them as the 'command' field in the container, rather than the 'args' field which is the default.
|
|--dry-run string[="unchanged"]Default: "none"|
||
Must be "none", "server", or "client". If client strategy, only print the object that would be sent, without sending it. If server strategy, submit server-side request without persisting the resource.
|
|--env strings|
||
Environment variables to set in the container.
|
|--expose --port|
||
If true, create a ClusterIP service associated with the pod. Requires --port.
|
|--field-manager stringDefault: "kubectl-run"|
||
Name of the manager used to track field ownership.
|
|-f, --filename strings|
||
to use to replace the resource.
|
|--force|
||
If true, immediately remove resources from API and bypass graceful deletion. Note that immediate deletion of some resources may result in inconsistency or data loss and requires confirmation.
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