---
doc_id: ref/docs-reference-kubectl-generated-kubectlapply-kubectlapplyedit-last-applied.md/docs-reference-kubectl-generated-kubectlapply-kubectlapplyedit-last-applied
chunk_id: ref/docs-reference-kubectl-generated-kubectlapply-kubectlapplyedit-last-applied.md/docs-reference-kubectl-generated-kubectlapply-kubectlapplyedit-last-applied#4-summary
chunk_level: summary
chunk_type: prose
heading: Synopsis
token_count: 118
summary: The default format is YAML. To edit in JSON, specify \"-o json\". The flag --windows-line-endings can be used to force Windows line endings, otherwise the default for your operating system will be...
---

The default format is YAML. To edit in JSON, specify "-o json".
The flag --windows-line-endings can be used to force Windows line endings, otherwise the default for your operating system will be used.
In the event an error occurs while updating, a temporary file will be created on disk that contains your unapplied changes. The most common error when updating a resource is another editor changing the resource on the server. When this occurs, you will have to apply your changes to the newer version of the resource, or update your temporary saved copy to include the latest resource version.