---
doc_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods
chunk_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods#19-summary
chunk_level: summary
chunk_type: prose
heading: Diagnosing the problem
token_count: 87
summary: `command` as `commnd` then the pod will be created but will not use the command line you intended it to use. The first thing to do is to delete your pod and try creating it again with the...
---

`command` as `commnd` then the pod will be created but
will not use the command line you intended it to use.
The first thing to do is to delete your pod and try creating it again with the `--validate` option.
For example, run `kubectl apply --validate -f mypod.yaml`.
If you misspelled `command` as `commnd` then will give an error like this: