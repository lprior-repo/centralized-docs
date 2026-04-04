---
doc_id: ref/docs-reference-kubectl-generated-kubectllabel.md/docs-reference-kubectl-generated-kubectllabel
chunk_id: ref/docs-reference-kubectl-generated-kubectllabel.md/docs-reference-kubectl-generated-kubectllabel#0-standard
chunk_level: standard
chunk_type: prose
heading: Synopsis
token_count: 408
summary: ## Table of Contents    - [Synopsis](#synopsis)   - [Examples](#examples) - [Update pod 'foo' with the label 'status' and the value 'unhealthy', overwriting any existing...
---

## Table of Contents

  - [Synopsis](#synopsis)
  - [Examples](#examples)
- [Update pod 'foo' with the label 'status' and the value 'unhealthy', overwriting any existing value](#update-pod-foo-with-the-label-status-and-the-value-unhealthy-overwriting-any-existing-value)
- [Update all pods in the namespace](#update-all-pods-in-the-namespace)
- [Update a pod identified by the type and name in "pod.json"](#update-a-pod-identified-by-the-type-and-name-in-podjson)
- [Update pod 'foo' only if the resource is unchanged from version 1](#update-pod-foo-only-if-the-resource-is-unchanged-from-version-1)
- [Update pod 'foo' by removing a label named 'bar' if it exists](#update-pod-foo-by-removing-a-label-named-bar-if-it-exists)
- [Does not require the --overwrite flag](#does-not-require-the---overwrite-flag)
  - [Options](#options)
  - [Parent Options Inherited](#parent-options-inherited)
  - [Feedback](#feedback)

---

## Synopsis
Update the labels on a resource.
* A label key and value must begin with a letter or number, and may contain letters, numbers, hyphens, dots, and underscores, up to 63 characters each.
* Optionally, the key can begin with a DNS subdomain prefix and a single '/', like example.com/my-app.
* If --overwrite is true, then existing labels can be overwritten, otherwise attempting to overwrite a label will result in an error.
* If --resource-version is specified, then updates will use this resource version, otherwise the existing resource-version will be used.
```
`kubectl label [--overwrite] (-f FILENAME | TYPE NAME) KEY\_1=VAL\_1 ... KEY\_N=VAL\_N [--resource-version=version]
`
```