---
doc_id: ref/docs-reference-kubectl-generated-kubectlconfig-kubectlconfigset.md/docs-reference-kubectl-generated-kubectlconfig-kubectlconfigset
chunk_id: ref/docs-reference-kubectl-generated-kubectlconfig-kubectlconfigset.md/docs-reference-kubectl-generated-kubectlconfig-kubectlconfigset#4-summary
chunk_level: summary
chunk_type: prose
heading: Synopsis
token_count: 114
summary: ## Synopsis Set an individual value in a kubeconfig file. PROPERTY\_NAME is a dot delimited name where each token represents either an attribute name or a map key. Map keys may not contain dots....
---

## Synopsis
Set an individual value in a kubeconfig file.
PROPERTY\_NAME is a dot delimited name where each token represents either an attribute name or a map key. Map keys may not contain dots.
PROPERTY\_VALUE is the new value you want to set. Binary fields such as 'certificate-authority-data' expect a base64 encoded string unless the --set-raw-bytes flag is used.
Specifying an attribute name that already exists will merge new fields on top of existing values.
```
`kubectl config set PROPERTY\_NAME PROPERTY\_VALUE
`
```