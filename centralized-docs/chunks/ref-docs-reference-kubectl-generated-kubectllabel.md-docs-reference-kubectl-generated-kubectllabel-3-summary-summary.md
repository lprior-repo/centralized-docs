---
doc_id: ref/docs-reference-kubectl-generated-kubectllabel.md/docs-reference-kubectl-generated-kubectllabel
chunk_id: ref/docs-reference-kubectl-generated-kubectllabel.md/docs-reference-kubectl-generated-kubectllabel#3-summary
chunk_level: summary
chunk_type: prose
heading: Synopsis
token_count: 121
summary: ## Synopsis Update the labels on a resource. * A label key and value must begin with a letter or number, and may contain letters, numbers, hyphens, dots, and underscores, up to 63 characters each. *...
---

## Synopsis
Update the labels on a resource.
* A label key and value must begin with a letter or number, and may contain letters, numbers, hyphens, dots, and underscores, up to 63 characters each.
* Optionally, the key can begin with a DNS subdomain prefix and a single '/', like example.com/my-app.
* If --overwrite is true, then existing labels can be overwritten, otherwise attempting to overwrite a label will result in an error.
* If --resource-version is specified, then updates will use this resource version, otherwise the existing resource-version will be used.