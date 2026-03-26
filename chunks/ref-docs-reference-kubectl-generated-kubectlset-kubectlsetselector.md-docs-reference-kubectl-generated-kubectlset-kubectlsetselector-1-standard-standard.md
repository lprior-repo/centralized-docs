---
doc_id: ref/docs-reference-kubectl-generated-kubectlset-kubectlsetselector.md/docs-reference-kubectl-generated-kubectlset-kubectlsetselector
chunk_id: ref/docs-reference-kubectl-generated-kubectlset-kubectlsetselector.md/docs-reference-kubectl-generated-kubectlset-kubectlsetselector#1-standard
chunk_level: standard
chunk_type: table
heading: Examples
token_count: 249
summary: # kubectl set selector Set the selector on a resource ## Synopsis Set the selector on a resource. Note that the new selector will overwrite the old selector if the resource had one prior to the...
---

# kubectl set selector
Set the selector on a resource
## Synopsis
Set the selector on a resource. Note that the new selector will overwrite the old selector if the resource had one prior to the invocation of 'set selector'.
A selector must begin with a letter or number, and may contain letters, numbers, hyphens, dots, and underscores, up to 63 characters. If --resource-version is specified, then updates will use this resource version, otherwise the existing resource-version will be used. Note: currently selectors can only be set on Service objects.
```
`kubectl set selector (-f FILENAME | TYPE NAME) EXPRESSIONS [--resource-version=version]
`
```
## Examples
```
` # Set the labels and selector before creating a deployment/service pair
kubectl create service clusterip my-svc --clusterip="None" -o yaml --dry-run=client | kubectl set selector --local -f - 'environment=qa' -o yaml | kubectl create -f -
kubectl create deployment my-dep --image=nginx -o yaml --dry-run=client | kubectl label --local -f - environment=qa -o yaml | kubectl create -f -
`
```