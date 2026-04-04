---
doc_id: ref/docs-reference-kubectl-generated-kubectlapply-kubectlapplyedit-last-applied.md/docs-reference-kubectl-generated-kubectlapply-kubectlapplyedit-last-applied
chunk_id: ref/docs-reference-kubectl-generated-kubectlapply-kubectlapplyedit-last-applied.md/docs-reference-kubectl-generated-kubectlapply-kubectlapplyedit-last-applied#5-summary
chunk_level: summary
chunk_type: prose
heading: Examples
token_count: 82
summary: ``` `kubectl apply edit-last-applied (RESOURCE/NAME | -f FILENAME) ` ``` ## Examples ``` ` # Edit the last-applied-configuration annotations by type/name in YAML kubectl apply edit-last-applied...
---

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