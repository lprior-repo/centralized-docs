---
doc_id: ref/docs-reference-kubectl-generated-kubectlalpha-kubectlalphakubercview.md/docs-reference-kubectl-generated-kubectlalpha-kubectlalphakubercview
chunk_id: ref/docs-reference-kubectl-generated-kubectlalpha-kubectlalphakubercview.md/docs-reference-kubectl-generated-kubectlalpha-kubectlalphakubercview#1-summary
chunk_level: summary
chunk_type: prose
heading: Examples
token_count: 121
summary: # kubectl alpha kuberc view Display the current kuberc configuration ## Synopsis Display the contents of the kuberc file in the specified output format. ``` `kubectl alpha kuberc view ` ``` ##...
---

# kubectl alpha kuberc view
Display the current kuberc configuration
## Synopsis
Display the contents of the kuberc file in the specified output format.
```
`kubectl alpha kuberc view
`
```
## Examples
```
` # View kuberc configuration in YAML format (default)
kubectl alpha kuberc view
# View kuberc configuration in JSON format
kubectl alpha kuberc view --output json
# View a specific kuberc file
kubectl alpha kuberc view --kuberc /path/to/kuberc
`
```