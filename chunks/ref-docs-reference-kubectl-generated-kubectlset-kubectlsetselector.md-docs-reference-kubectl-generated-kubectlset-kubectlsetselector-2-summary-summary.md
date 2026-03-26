---
doc_id: ref/docs-reference-kubectl-generated-kubectlset-kubectlsetselector.md/docs-reference-kubectl-generated-kubectlset-kubectlsetselector
chunk_id: ref/docs-reference-kubectl-generated-kubectlset-kubectlsetselector.md/docs-reference-kubectl-generated-kubectlset-kubectlsetselector#2-summary
chunk_level: summary
chunk_type: prose
heading: Synopsis
token_count: 127
summary: ## Synopsis Set the selector on a resource. Note that the new selector will overwrite the old selector if the resource had one prior to the invocation of 'set selector'. A selector must begin with a...
---

## Synopsis
Set the selector on a resource. Note that the new selector will overwrite the old selector if the resource had one prior to the invocation of 'set selector'.
A selector must begin with a letter or number, and may contain letters, numbers, hyphens, dots, and underscores, up to 63 characters. If --resource-version is specified, then updates will use this resource version, otherwise the existing resource-version will be used. Note: currently selectors can only be set on Service objects.
```
`kubectl set selector (-f FILENAME | TYPE NAME) EXPRESSIONS [--resource-version=version]
`
```