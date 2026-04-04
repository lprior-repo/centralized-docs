---
doc_id: ref/docs-reference-kubectl-generated-kubectlapply-kubectlapplyedit-last-applied.md/docs-reference-kubectl-generated-kubectlapply-kubectlapplyedit-last-applied
chunk_id: ref/docs-reference-kubectl-generated-kubectlapply-kubectlapplyedit-last-applied.md/docs-reference-kubectl-generated-kubectlapply-kubectlapplyedit-last-applied#1-standard
chunk_level: standard
chunk_type: prose
heading: Examples
token_count: 341
summary: # kubectl apply edit-last-applied Edit latest last-applied-configuration annotations of a resource/object ## Synopsis Edit the latest last-applied-configuration annotations of resources from the...
---

# kubectl apply edit-last-applied
Edit latest last-applied-configuration annotations of a resource/object
## Synopsis
Edit the latest last-applied-configuration annotations of resources from the default editor.
The edit-last-applied command allows you to directly edit any API resource you can retrieve via the command-line tools. It will open the editor defined by your KUBE\_EDITOR, or EDITOR environment variables, or fall back to 'vi' for Linux or 'notepad' for Windows. You can edit multiple objects, although changes are applied one at a time. The command accepts file names as well as command-line arguments, although the files you point to must be previously saved versions of resources.
The default format is YAML. To edit in JSON, specify "-o json".
The flag --windows-line-endings can be used to force Windows line endings, otherwise the default for your operating system will be used.
In the event an error occurs while updating, a temporary file will be created on disk that contains your unapplied changes. The most common error when updating a resource is another editor changing the resource on the server. When this occurs, you will have to apply your changes to the newer version of the resource, or update your temporary saved copy to include the latest resource version.
```
`kubectl apply edit-last-applied (RESOURCE/NAME | -f FILENAME)
`
```
## Examples
```
` # Edit the last-applied-configuration annotations by type/name in YAML
kubectl apply edit-last-applied deployment/nginx
# Edit the last-applied-configuration annotations by file in JSON
kubectl apply edit-last-applied -f deploy.yaml -o json
`
```