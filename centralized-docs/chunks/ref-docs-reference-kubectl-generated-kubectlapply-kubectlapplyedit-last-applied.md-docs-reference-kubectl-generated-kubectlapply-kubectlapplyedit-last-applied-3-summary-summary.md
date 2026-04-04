---
doc_id: ref/docs-reference-kubectl-generated-kubectlapply-kubectlapplyedit-last-applied.md/docs-reference-kubectl-generated-kubectlapply-kubectlapplyedit-last-applied
chunk_id: ref/docs-reference-kubectl-generated-kubectlapply-kubectlapplyedit-last-applied.md/docs-reference-kubectl-generated-kubectlapply-kubectlapplyedit-last-applied#3-summary
chunk_level: summary
chunk_type: prose
heading: Synopsis
token_count: 128
summary: Edit the latest last-applied-configuration annotations of resources from the default editor. The edit-last-applied command allows you to directly edit any API resource you can retrieve via the...
---

Edit the latest last-applied-configuration annotations of resources from the default editor.
The edit-last-applied command allows you to directly edit any API resource you can retrieve via the command-line tools. It will open the editor defined by your KUBE\_EDITOR, or EDITOR environment variables, or fall back to 'vi' for Linux or 'notepad' for Windows. You can edit multiple objects, although changes are applied one at a time. The command accepts file names as well as command-line arguments, although the files you point to must be previously saved versions of resources.
The default format is YAML. To edit in JSON, specify