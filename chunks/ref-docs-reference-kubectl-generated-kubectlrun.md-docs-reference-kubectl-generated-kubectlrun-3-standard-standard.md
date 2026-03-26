---
doc_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun
chunk_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun#3-standard
chunk_level: standard
chunk_type: table
heading: Options
token_count: 510
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
|-k, --