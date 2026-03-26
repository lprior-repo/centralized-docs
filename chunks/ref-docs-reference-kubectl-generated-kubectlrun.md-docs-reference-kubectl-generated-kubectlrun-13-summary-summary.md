---
doc_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun
chunk_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun#13-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 128
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
|--cascade string[